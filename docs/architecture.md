# Architecture

## Data Structures

### Link

A `String` of the domain that represents the website, for example `"www.google.com" or "blog.myspecialplace.com"`.

[See here](https://developer.mozilla.org/en-US/docs/Learn/Common_questions/What_is_a_URL) for a good explanation of the different pieces in the URL:
<br/><img height=70 src="../docs/assets/URL_description.png" alt="Structure and components of a URL"></img>

_Right now all voting just happens on the domain, but there could be a future where Discontent allows voting on individual paths underneath the domain. This would allow voting on individual articles on Medium for example._

### Vote

An `Integer` that's either a +1 or -1. Usually stored in a `Tuple` along with the timestamp when the vote was made `Tuple(Vote, Timestamp)`

### Score

An `enum` that represents how good a website `Link` is. It has 4 possible values:

1. `Good`
1. `Bad`
1. `Controversial`
1. `NoScore`

The score for a `Link` is calculated in the API and exposed to the extension through the `/scores` request. The definition is:

- `Good` = Sum of all votes >= 10
- `Bad` = Sum of all votes <= -10
- `Controversial` = -10 < Sum of all votes < 10 && Number of votes > 10
- `NoScore` = None of the above

### Users Collection

| Property | Type                                 | Default               | Description                              |
| -------- | ------------------------------------ | --------------------- | ---------------------------------------- |
| `userId` | `UUID` (Primary Key)                 | <generated_by_client> | Login key for every user. Don't share it |
| `votes`  | `Hash<Link, Tuple(Vote, Timestamp)>` |                       |                                          |

### Links Collection

| Property        | Type                                     | Default | Description |
| --------------- | ---------------------------------------- | ------- | ----------- |
| `link`          | `Link` (Primary Key)                     |         |             |
| `numberOfVotes` | `Integer`                                | 0       |             |
| `sumOfVotes`    | `Integer`                                | 0       |             |
| `voters`        | `Hash<UserUUID, Tuple(Vote, Timestamp)>` | Empty   |             |

## API

| Request                                               | Response                   |
| ----------------------------------------------------- | -------------------------- |
| `GET /scores?links=<JSON stringified array of links>` | `Array<Tuple(Link,Score)>` |
| `POST /vote?link=<Link>&user=<UUID>&vote=<Vote>`      |                            |

## Database

I decided to go with a NoSQL database for two reasons:

1. It'd be cool to learn
1. My extremely basic understanding of NoSQL leads me to believe that it's better suited for what Discontent is trying to do.
   DynamoDB on AWS seems cheap enough and if this extension actually gets used and needs to scale then future Tom won't be boned.

The access patterns are pretty well defined:

| Version 1 Access Patterns                    | Description                                      |
| -------------------------------------------- | ------------------------------------------------ |
|                                              |                                                  |
| Get the score for a given hostname           |                                                  |
| Delete a user and undo all their votes       |                                                  |
| Create a user vote for a given hostname      | When a vote button is clicked                    |
| Update a user vote for a given hostname      | When a vote button is clicked again              |
| Get top hostnames by score                   | To create a leaderboard of the best hostnames    |
| Get users by number of votes                 | To do analyses on possible abuse, best voters... |
| Get list of vote timestamps for a given user | To check they aren't abusing the system          |

Future versions of Discontent might want to support:

| Version 2 Access Patterns               | Description                            |
| --------------------------------------- | -------------------------------------- |
|                                         |                                        |
| Read a user vote for a given hostname   | To auto select the correct vote button |
| Delete a user vote for a given hostname | When a vote button is deselected       |
