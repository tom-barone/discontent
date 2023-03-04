import { Score, ScoresRequest, ScoresResponse } from "./types";

const ENDPOINT =
  process.env.NODE_ENV === "production"
    ? " https://5h7cyehfq1.execute-api.us-east-1.amazonaws.com/v1"
    : "http://localhost:9000/lambda-url/request-handler/v1";

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
  const response = await fetch(url, {
    method: "POST",
    body: JSON.stringify({
      link: {
        hostname,
      },
      value,
      user_id,
    }),
  });
  return response.ok;
}
