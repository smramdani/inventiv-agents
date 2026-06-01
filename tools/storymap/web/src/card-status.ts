import type { CardStatus } from "./types";

export function cardStatusLabel(s: CardStatus): string {
  switch (s) {
    case "shipped":
      return "Shipped";
    case "in_progress":
      return "In progress";
    case "deferred":
      return "Deferred";
    default:
      return "Planned";
  }
}
