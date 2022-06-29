<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Search from "$lib/search/Search.svelte";
	import StateCards from "$lib/cards/state-cards.svelte";
</script>

<svelte:head>
	<title>State Budget Search</title>
	<meta name="State budget search" content="Budget search for U.S. states" />
</svelte:head>

<Search customClasses="bg-gray-700 p-6" />
<div class="bg-gray-700">
	<StateCards customClasses="grid grid-cols-3" />
</div>
