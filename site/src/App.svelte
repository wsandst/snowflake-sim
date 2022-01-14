<script>
	// Emulator
	//import binaryen from "binaryen"
	export let snowflakeSimLib;
	let simCtx;

	function test() {
		simCtx = snowflakeSimLib.SnowflakeSimContext.new();
		console.log(simCtx.get_str());
	}

</script>

<svelte:head>
	<title>Snowflake Simulation</title>
</svelte:head>

<main >
	<button on:click={test}/>
</main>

<style>
	:global(html) {
		height: 100%;
		background-color: #202020;
		color: #fff;
	}

	:global(body) {
		margin: 0;
    	height: 100%;
	}

	main {
		display: block;
		height: 100%;
		width: 100%;
	}

	button {
		width: 300px;
		height: 100px;
	}
</style>