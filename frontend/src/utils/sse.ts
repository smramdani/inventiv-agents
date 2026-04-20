/**
 * POST + ND-SSE body reader (not EventSource — browser EventSource is GET-only).
 */
export async function consumeSsePost(
  url: string,
  headers: Record<string, string>,
  body: string,
  onEvent: (event: string, data: unknown) => void,
  signal?: AbortSignal,
): Promise<void> {
  const res = await fetch(url, {
    method: "POST",
    headers: { ...headers, "Content-Type": "application/json" },
    body,
    signal,
  });
  if (!res.ok) {
    const t = await res.text();
    throw new Error(t || res.statusText);
  }
  const reader = res.body?.getReader();
  if (!reader) {
    throw new Error("No response body");
  }
  const dec = new TextDecoder();
  let buf = "";
  while (true) {
    const { done, value } = await reader.read();
    if (done) {
      break;
    }
    buf += dec.decode(value, { stream: true });
    for (;;) {
      const sep = buf.indexOf("\n\n");
      if (sep < 0) {
        break;
      }
      const chunk = buf.slice(0, sep);
      buf = buf.slice(sep + 2);
      let eventName = "message";
      const dataLines: string[] = [];
      for (const line of chunk.split("\n")) {
        if (line.startsWith("event:")) {
          eventName = line.slice(6).trim();
        } else if (line.startsWith("data:")) {
          dataLines.push(line.slice(5).trimStart());
        }
      }
      const dataStr = dataLines.join("\n");
      let data: unknown = {};
      if (dataStr) {
        try {
          data = JSON.parse(dataStr) as unknown;
        } catch {
          data = { raw: dataStr };
        }
      }
      onEvent(eventName, data);
    }
  }
}
