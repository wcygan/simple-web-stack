import { JSX } from "preact";
import { SearchParams } from "../types.ts";

interface SearchFormProps {
  searchParams: SearchParams;
  onSearchChange: (params: SearchParams) => void;
  onClearSearch: () => void;
}

export function SearchForm({ searchParams, onSearchChange, onClearSearch }: SearchFormProps): JSX.Element {
  const handleQueryChange = (e: Event) => {
    const target = e.target as HTMLInputElement;
    onSearchChange({
      ...searchParams,
      q: target.value
    });
  };

  const handleStatusChange = (e: Event) => {
    const target = e.target as HTMLSelectElement;
    onSearchChange({
      ...searchParams,
      status: target.value
    });
  };

  const handleClear = () => {
    onClearSearch();
  };

  return (
    <div class="search-form bg-white p-4 rounded-lg shadow-sm border border-gray-200 mb-6">
      <div class="flex flex-col sm:flex-row gap-4 items-start sm:items-end">
        {/* Search Query Input */}
        <div class="flex-1 min-w-0">
          <label htmlFor="search-query" class="block text-sm font-medium text-gray-700 mb-1">
            Search Tasks
          </label>
          <input
            id="search-query"
            type="text"
            value={searchParams.q || ""}
            onInput={handleQueryChange}
            placeholder="Search by title..."
            class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm 
                   focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500
                   placeholder-gray-400"
          />
        </div>

        {/* Status Filter */}
        <div class="w-full sm:w-48">
          <label htmlFor="status-filter" class="block text-sm font-medium text-gray-700 mb-1">
            Status
          </label>
          <select
            id="status-filter"
            value={searchParams.status || "all"}
            onChange={handleStatusChange}
            class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm 
                   focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500
                   bg-white"
          >
            <option value="all">All Tasks</option>
            <option value="pending">Pending</option>
            <option value="completed">Completed</option>
          </select>
        </div>

        {/* Clear Button */}
        <div class="w-full sm:w-auto">
          <button
            onClick={handleClear}
            class="w-full sm:w-auto px-4 py-2 text-sm font-medium text-gray-700 
                   bg-gray-100 border border-gray-300 rounded-md hover:bg-gray-200 
                   focus:outline-none focus:ring-2 focus:ring-gray-500 focus:border-gray-500
                   transition-colors duration-200"
          >
            Clear
          </button>
        </div>
      </div>

      {/* Active Filters Display */}
      {(searchParams.q || (searchParams.status && searchParams.status !== "all")) && (
        <div class="mt-3 pt-3 border-t border-gray-200">
          <div class="flex flex-wrap gap-2 items-center">
            <span class="text-sm text-gray-600">Active filters:</span>
            
            {searchParams.q && (
              <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                Search: "{searchParams.q}"
              </span>
            )}
            
            {searchParams.status && searchParams.status !== "all" && (
              <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
                Status: {searchParams.status}
              </span>
            )}
          </div>
        </div>
      )}
    </div>
  );
}