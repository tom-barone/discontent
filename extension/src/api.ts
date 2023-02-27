import { Score, ScoresRequest, ScoresResponse } from "./types";

export async function fetchScores(
  request: ScoresRequest
): Promise<ScoresResponse> {
  const params = new URLSearchParams();

  params.append("from", request.json);
  // TODO: Add production / development handling here
  const url =
    "http://localhost:9000/lambda-url/request-handler/v1/scores?" + params;
  const response = await window.fetch(url, {
    method: "GET",
    headers: {
      "content-type": "application/json",
    },
  });
  const scores = await response.json();

  // Try parse the response into a ScoresResponse
  // TODO: Add error handling for parse failures
  const result: ScoresResponse = new Map<string, Score>();
  scores.forEach((link_score: any) => {
    result.set(link_score.link.hostname, link_score.score);
  });
  return result;
}
