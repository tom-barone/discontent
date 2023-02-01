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

A `String` of the domain that represents the website, for example `"www.google.com" or "blog.myspecialplace.com"`.

[See here](https://developer.mozilla.org/en-US/docs/Learn/Common_questions/What_is_a_URL) for a good explanation of the different pieces in the URL:
<br/><img height=70 src="../docs/assets/URL_description.png" alt="Structure and components of a URL"></img>

_Right now all voting just happens on the domain, but there could be a future where Discontent allows voting on paths underneath the domain. This mean being able to vote on individual Medium articles for example._

### Vote

An `Integer` that's either a +1 or -1. Sum all votes to get the `Score` of a `Link`.

### Score

An `enum` that represents how good a website `Link` is. It has 4 possible values:

| Enum            | Definition                                              |
| --------------- | ------------------------------------------------------- |
| `Good`          | Sum of all votes >= 10                                  |
| `Bad`           | Sum of all votes <= -10                                 |
| `Controversial` | (-10 < Sum of all votes < 10) && (Number of votes > 10) |
| `NoScore`       | If none of the above                                    |

The score is calculated in the API and exposed to the extension through the `/scores` request.

### User

Identified by a `UUID`. I wanted a passwordless system and this seemed like a flexible choice. Has a number of properties:

- userNotes: `String`
- banned: `Boolean`

Only one `Vote` can be submitted per `User` per `Link`.

## API

_TODO: Link to a swagger page._

| Request                                                               | Response                       | Visibility                            |
| --------------------------------------------------------------------- | ------------------------------ | ------------------------------------- |
| `GET /scores?links=[l1, l2, ...]`                                     | `[{link: Link, score: Score}]` | Public                                |
| `GET /vote?link=link`                                                 | `Vote`                         | Public                                |
| `POST /vote {link:<Link>, vote:<Vote>}`                               |                                | Public                                |
| `POST /admin/vote {link:<Link>, vote:<Vote>, timestamp: <Timestamp>}` |                                | Admin only, used for database seeding |

## Database

I decided to go with a NoSQL database for two reasons:

1. It'd be cool to learn.
1. My extremely basic understanding of NoSQL leads me to believe that it's better suited for what Discontent is trying to do.
   DynamoDB on AWS seems cheap enough and if this extension actually gets used and needs to scale then future Tom won't be boned.

The access patterns are reasonably well defined:

| Runtime Access Patterns       | Description                                        | Table - Filter                                            |
| ----------------------------- | -------------------------------------------------- | --------------------------------------------------------- |
| Get vote summaries for a Link | Summaries are `sumOfVotes` & `numberOfVotes`       | `Table:Discontent - PK=link#<link>, SK=link#<link>`       |
| Get all votes for a Link      | To calculate `sumOfVotes` & `numberOfVotes`        | `Table:Discontent - PK=link#<link>, SK.startswith(user#)` |
| Get vote for a Link and user  | To make sure a user can't vote twice               | `Table:Discontent - PK=link#<link>, SK=user#<userId>`     |
| Get vote for a Link and user  | To auto select the correct vote button             | `Table:Discontent - PK=link#<link>, SK=user#<userId>`     |
| Get vote summaries for a User | To limit the number of submissions in a time range | `Table:Discontent - PK=day#<date>, SK=user#<userId>`      |
| Get banned state for a User   | Prevent banned users from submitting more votes    | `Table:Discontent - PK=user#<userId>, SK=user#<userId>`   |

The following are analysis access patterns, not really part of regular usage.

| Analysis Access Patterns              | Description                                 | Table & Filter                                             |
| ------------------------------------- | ------------------------------------------- | ---------------------------------------------------------- |
| Get User details                      | To carry out abuse investigations           | `Table:Discontent - PK=user#<userId>, SK=user#<userId>,`   |
| Get all votes for a user              | To carry out abuse investigations           | `GSI:UserVotes - PK=user#<userId>, SK.within(timerange)`   |
| Get top users by daily count of votes | To identify possible abuse                  | `GSI:DailyUserHistory - PK=<day>, SK.top(N)`               |
| Get top links by daily count of votes | To identify possible abuse                  | `GSI:DailyLinkHistoryByCountOfVotes - PK=<day>, SK.top(N)` |
| Get top links by daily sum of votes   | To create a best links leaderboard          | `GSI:DailyLinkHistoryBySumOfVotes - PK=<day>, SK.top(N)`   |
| Get top links by daily count of votes | To create a controversial links leaderboard | `GSI:DailyLinkHistoryByCountOfVotes - PK=<day>, SK.top(N)` |

### Schema visualisations

_TODO: Add images from the NoSQL workbench_

## Sequence diagrams

### Get scores for links

```mermaid
sequenceDiagram
    actor Extension
    participant API
    participant Database
    Extension->>API: GET /scores?links=[l1, l2, ...] AuthHeader=user
		activate API
		API->>API: Validate request
    alt Request Error
        API->>Extension: Request Error (Invalid params / authentication...)
    end
		API->>Database: BatchGetItem(Table:Discontent - PK=<l1, l2...>, SK=<l1, l2...>)
		Note over API,Database: If a link does not yet exist in the table, it's not returned
		activate Database
    alt Database Error
        Database->>API: Database Error (connection / server...)
        API->>Extension: Server Error
    end
		Database->>API: Return [{link, sumOfVotes, numberOfVotes}]
		API->>API: Calculate scores for [{link, sumOfVotes, numberOfVotes}]
		API->>Extension: Return [{link, score}]
    deactivate Database
    deactivate API
```

### Submit a vote for a link

```mermaid
sequenceDiagram
    actor Extension
    participant API
    participant Database
    Extension->>API: POST /vote?link=link&vote=vote AuthHeader=user
		activate API
		API->>API: Validate parameters
    alt Invalid parameters
        API->>Extension: Error: Invalid parameters
    end
		API->>Database: Check user history. GetBatchItems(_________________)
		Note over API,Database: Too many votes? GetItem(PK=date, SK=userId)
		Note over API,Database: Banned? GetItem(PK=userId, SK=userId)
		activate Database
    alt Database Error
        Database->>API: Database Error (connection / server...)
        API->>Extension: Server Error
    end
		Database->>API: Return UserHistory
		deactivate Database
		API->>API: Check UserHistory
    alt Failed
        API->>Extension: Error: Too many votes or banned
    end
		activate Database
		API->>Database: Submit vote. BatchWriteItems(_________________)
		Note over API,Database: UpdateItem(PK=link, SK=userId | vote)
		Note over API,Database: UpdateItem(PK=link, SK=link | countOfVotes++, sumOfVotes+=vote)
		Note over API,Database: UpdateItem(PK=day, SK=link | countOfVotes++, sumOfVotes+=vote)
		Note over API,Database: UpdateItem(PK=day, SK=user | countOfVotes++, sumOfVotes+=vote)
		Note over API,Database: UpdateItem(PK=user, SK=user | userNotes, banned=false)
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
