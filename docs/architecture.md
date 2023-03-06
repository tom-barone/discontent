# Architecture

## Data Structures

```mermaid
%%{ init: { "er" : { "layoutDirection" : "LR" } } }%%
erDiagram
    User ||--o{ Vote : submits
    Vote o{--|| Link : on
    Link o{--|| Score : has
```

### Link

The hostname `String` that represents the website, for example `"www.google.com" or "blog.myspecialplace.com"`. [See here](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname) for a good explanation of the different pieces in the URL:
<br/><img height=70 src="../docs/assets/URL_description.png" alt="Structure and components of a URL"></img>

_Right now all voting just happens on the hostname, but there could be a future where voting happens on full URL paths. Like voting on individual Medium articles for example._

### Vote

An `Integer` that's either a +1 or -1, stored with a `Timestamp` when the vote was made. Always associated with a `User` and a `Link`.

### Timestamp

Represented everywhere as an [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339) string with the specific format `2023-02-02T09:36:03Z`.

### Score

An `enum` that represents how good a website `Link` is. It has 4 possible values:

| Enum            | Definition                                              |
| --------------- | ------------------------------------------------------- |
| `Good`          | Sum of all votes >= 20                                  |
| `Bad`           | Sum of all votes <= -10                                 |
| `Controversial` | (-10 < Sum of all votes < 20) && (Number of votes > 50) |
| `NoScore`       | If none of the above                                    |

The score is calculated in the API and exposed to the extension through the `/scores` request.

In the future this will probably need to be tweaked for more nuanced scoring, like weighting recent votes higher.

### User

Identified by a `UUID`. I wanted a passwordless system and this seemed like a flexible choice. Has a number of properties:

- is_banned: `Boolean`
- created_at: `Timestamp`

### Settings

System wide configuration that can change the behaviour of everything.

- voting_is_disabled: `Boolean`
- maximum_votes_per_user_per_day: 10

The idea behind `voting_is_disabled` is in case there's a spam armaggedon and all voting needs to be stopped.

## API

| Request                               | Response                       |
| ------------------------------------- | ------------------------------ |
| `GET /scores?for=[link1, link2, ...]` | `[{link: Link, score: Score}]` |
| `POST /vote {link, vote, user_id}`    |                                |

## Database

I decided to go with a NoSQL database for two reasons:

1. It'd be cool to learn.
1. My extremely basic understanding of NoSQL leads me to believe that it's better suited for what this is trying to do.
   DynamoDB on AWS seems cheap enough and if this extension actually gets used and needs to scale then future Tom won't be boned.

The access patterns are reasonably well defined:

| Runtime Access Patterns       | Description                                        | Table - Filter                                            |
| ----------------------------- | -------------------------------------------------- | --------------------------------------------------------- |
| Get vote summaries for a Link | Summaries are `sum_of_votes` & `count_of_votes`    | `Table:Discontent - PK=link#<link>, SK=link#<link>`       |
| Get all votes for a Link      | To calculate `sum_of_votes` & `count_of_votes`     | `Table:Discontent - PK=link#<link>, SK.startswith(user#)` |
| Get vote for a Link and user  | To make sure a user can't vote twice               | `Table:Discontent - PK=link#<link>, SK=user#<user_id>`    |
| Get vote for a Link and user  | To auto select the correct vote button             | `Table:Discontent - PK=link#<link>, SK=user#<user_id>`    |
| Get vote summaries for a User | To limit the number of submissions in a time range | `Table:Discontent - PK=day#<date>, SK=user#<user_id>`     |
| Get banned state for a User   | Prevent banned users from submitting more votes    | `Table:Discontent - PK=user#<user_id>, SK=user#<user_id>` |

The following are analysis access patterns, not really part of regular usage.

| Analysis Access Patterns              | Description                                 | Table & Filter                                             |
| ------------------------------------- | ------------------------------------------- | ---------------------------------------------------------- |
| Get User details                      | To carry out abuse investigations           | `Table:Discontent - PK=user#<user_id>, SK=user#<user_id>,` |
| Get all votes for a user              | To carry out abuse investigations           | `GSI:UserVotes - PK=user#<user_id>, SK.within(timerange)`  |
| Get top users by daily count of votes | To identify possible abuse                  | `GSI:DailyUserHistory - PK=<day>, SK.top(N)`               |
| Get top links by daily count of votes | To identify possible abuse                  | `GSI:DailyLinkHistoryByCountOfVotes - PK=<day>, SK.top(N)` |
| Get top links by daily sum of votes   | To create a best links leaderboard          | `GSI:DailyLinkHistoryBySumOfVotes - PK=<day>, SK.top(N)`   |
| Get top links by daily count of votes | To create a controversial links leaderboard | `GSI:DailyLinkHistoryByCountOfVotes - PK=<day>, SK.top(N)` |

## Sequence diagrams

### Get scores for links

```mermaid
sequenceDiagram
    actor Extension
    participant API
    participant Database
    Extension->>API: GET /scores?for=[link1, link2, ...]
		activate API
		API->>API: Validate request
    alt Request Error
        API->>Extension: Request Error (Invalid params / authentication...)
    end
		API->>Database: BatchGetItem(Table:Discontent - PK,SK=<link, ...>)
		Note over API,Database: If a link does not yet exist in the table, it's not returned
		activate Database
    alt Database Error
        Database->>API: Database Error (connection / server...)
        API->>Extension: Server Error
    end
		Database->>API: Return [{link, sum_of_votes, count_of_votes}, ...]
		API->>API: Calculate scores
		API->>Extension: Return [{link, score}, ...]
    deactivate Database
    deactivate API
```

### Submit a vote for a link

```mermaid
sequenceDiagram
    actor Extension
    participant API
    participant Database
    Extension->>API: POST /vote {link, vote, user_id}`
		activate API
		API->>API: Validate parameters
    alt Invalid parameters
        API->>Extension: Error: Invalid parameters
    end
		API->>Database: Check user history & settings. GetBatchItems(___________)
		Note over API,Database: Voting disabled? GetItem(PK=settings, SK=settings)
		Note over API,Database: Too many votes? GetItem(PK=date, SK=user_id)
		Note over API,Database: User exists? User banned? GetItem(PK=user_id, SK=user_id)
		Note over API,Database: Already voted? GetItem(PK=link, SK=user_id)
		activate Database
    alt Database Error
        Database->>API: Database Error (connection / server...)
        API->>Extension: Server Error
    end
		Database->>API: Return UserHistory & Settings
		deactivate Database
		API->>API: Check user history & Settings
    alt Failed
        API->>Extension: 403 Forbidden: Too many votes / banned / voting disabled
    end
		activate Database
		API->>Database: Submit vote. BatchWriteItems(_________________)
    alt If user does not exist
		Note over API,Database: Put(PK=user_id, SK=user_id | not_banned,created_at)
		Note over API,Database: <run [First time user voting on link]>
		else First time user voting on link
		Note over API,Database: Put(PK=link, SK=user_id | vote)
		Note over API,Database: Update(PK=link, SK=link | count_of_votes++, sum_of_votes+=vote)
		Note over API,Database: -- Add history
		Note over API,Database: Update(PK=day, SK=link | count++, sum+=vote)
		Note over API,Database: Update(PK=day, SK=user | count++, sum+=vote)
    else If user already voted on link
		Note over API,Database: Put(PK=link, SK=user_id | vote)
		Note over API,Database: Update(PK=link, SK=link | sum_of_votes+=(-old_vote+new_vote))
		Note over API,Database: -- Undo old history
		Note over API,Database: Update(PK=old_day, SK=link | count--, sum-=old_vote)
		Note over API,Database: Update(PK=old_day, SK=user | count--, sum-=old_vote)
		Note over API,Database: -- Add history
		Note over API,Database: Update(PK=day, SK=link | count++, sum+=vote)
		Note over API,Database: Update(PK=day, SK=user | count++, sum+=vote)
		end
		activate Database
    alt Database Error
        Database->>API: Database Error (connection / server...)
        API->>Extension: Server Error
    end
		Database->>API: Return success
		API->>Extension: Return success
    deactivate Database
    deactivate API
```
