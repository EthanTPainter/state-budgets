<script lang="ts">
	export let customClasses: string = "";
	import stateCards from "../../data/states/cards";
	import { stateCardSearch } from "../../store";

	let filter: string;

	stateCardSearch.subscribe((value) => {
		filter = value;
	});

	$: filteredList = stateCards.filter(stateName => stateName.toLowerCase().includes(filter.toLowerCase()));
</script>

<section class="min-h-screen">
	<div class={customClasses}>
		{#each filteredList as state}
			<a
				href="/{state}"
				class="flex flex-col items-center bg-white rounded-lg border shadow-md md:flex-row md:max-w-xl hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-green-700"
			>
				<img
					class="object-cover w-full h-96 rounded-t-lg md:h-auto md:w-48 md:rounded-none md:rounded-l-lg text-white"
					src="/docs/images/blog/image-4.jpg"
					alt="State of {state}"
				/>
				<div class="flex flex-col justify-between p-4 leading-normal">
					<h5
						class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white"
					>
						{state}
					</h5>
					<p class="mb-3 font-normal text-gray-700 dark:text-gray-400">
						2022 {state} Budget
					</p>
				</div>
			</a>
		{/each}
	</div>
</section>
