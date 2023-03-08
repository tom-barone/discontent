import { Score, ScoresRequest, ScoresResponse } from "./types";

const ENDPOINT = process.env.LAMBDA_API_URL;

export async function fetchScores(
  request: ScoresRequest
): Promise<ScoresResponse> {
  const params = new URLSearchParams();

  params.append("from", request.json);
  const url = ENDPOINT + "/scores?" + params;
  const response = await fetch(url, {
    method: "GET",
  });
  const scores = await response.json();
  // Try parse the response into a ScoresResponse
  // TODO: Add error handling for parse failures
  const result = new Map<string, Score>();
  scores.forEach((link_score: any) => {
    result.set(link_score.link.hostname, link_score.score);
  });
  return Object.fromEntries(result);
}

export async function submitVote(
  value: 1 | -1,
  hostname: string,
  user_id: string
): Promise<boolean> {
  const url = ENDPOINT + "/vote";
  return fetch(url, {
    method: "POST",
    body: JSON.stringify({
      link: {
        hostname,
      },
      value,
      user_id,
    }),
  })
    .catch((error) => {
      console.error(error);
      return Promise.reject("Could not connect to the Discontent API");
    })
    .then(async (response) => {
      return [response.status, await response.text()];
    })
    .then(([status, response_object]) => {
      if (status === 200) {
        return true;
      } else {
        console.error(`Request error ${status}`, response_object);
        // TODO:
        // 		Add nice user formatted messages to the lambda response
        // 		e.g. {error: "<full message>", user_message: "<user friendly message>"}
        // 		Return the user message here
        // 		console.error the full message
        return Promise.reject("Could not submit your vote");
      }
    });
}
