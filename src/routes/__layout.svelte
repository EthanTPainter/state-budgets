<script lang="ts">
	import "../app.css";
	import Header from "$lib/header/Header.svelte";
	import Search from "$lib/search/Search.svelte";
	import StateCards from "$lib/cards/state-cards.svelte";
</script>

<Header></Header>
<Search customClasses="bg-gray-700 p-6"></Search>
<div class="bg-gray-700">
	<StateCards customClasses="grid grid-cols-3"></StateCards>
</div>
