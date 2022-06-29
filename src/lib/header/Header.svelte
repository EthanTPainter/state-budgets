<script lang="ts">
	import svelteLogo from "./svelte-logo.svg";
	import githubLogo from "./github-logo.svg";
</script>

<header class="bg-gray-700">
	<div class="corner">
		<a href="https://kit.svelte.dev">
			<img src={svelteLogo} alt="SvelteKit" />
		</a>
	</div>

	<div class="corner">
		<a href="https://github.com/EthanTPainter/state-budgets">
			<img src={githubLogo} alt="GitHub" />
		</a>
	</div>
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
	}

	.corner {
		width: 3em;
		height: 3em;
	}

	.corner a {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
	}

	.corner img {
		margin: 2em;
		width: 2em;
		height: 2em;
		object-fit: contain;
	}
</style>
