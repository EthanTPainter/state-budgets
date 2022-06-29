<script lang="ts">
	export let customClasses = "";
	import { stateCardSearch } from "../../store";

	function updateSearchFilter(event: any) {
		stateCardSearch.update(() => event.target.value);
	}
</script>

<form class={customClasses}>
	<form class="flex justify-center">
		<input
			on:input={updateSearchFilter}
			type="search"
			id="default-search"
			class="p-5 w-1/3 self-center text-lg bg-gray-800 text-gray-900 rounded-lg border-4 border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
			placeholder="Enter State Here"
			required
		/>
	</form>
</form>
