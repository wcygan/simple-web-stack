import { createDefine } from "fresh";
import type { PaginationParams } from "./types.ts";

// deno-lint-ignore no-empty-interface
export interface State {}

export const define = createDefine<State>();

export function buildPaginationQuery(params: PaginationParams): URLSearchParams {
  const query = new URLSearchParams();
  
  if (params.page !== undefined) {
    query.set("page", params.page.toString());
  }
  
  if (params.limit !== undefined) {
    query.set("limit", params.limit.toString());
  }
  
  if (params.offset !== undefined) {
    query.set("offset", params.offset.toString());
  }
  
  return query;
}

export function calculateTotalPages(total: number, limit: number): number {
  return Math.ceil(total / limit);
}

export function getPageRange(currentPage: number, totalPages: number, maxVisible = 5): number[] {
  const start = Math.max(1, currentPage - Math.floor(maxVisible / 2));
  const end = Math.min(totalPages, start + maxVisible - 1);
  
  const pages: number[] = [];
  for (let i = start; i <= end; i++) {
    pages.push(i);
  }
  
  return pages;
}

export function createPaginationInfo(page: number, limit: number, total: number) {
  const totalPages = calculateTotalPages(total, limit);
  
  return {
    total,
    page,
    limit,
    total_pages: totalPages,
    has_next: page < totalPages,
    has_prev: page > 1,
  };
}
