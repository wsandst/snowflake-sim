<script>
	import Display from './Display.svelte'
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa'
	import { faPause, faPlay, faDownload } from '@fortawesome/free-solid-svg-icons'
	
	export let snowflakeSimLib;
	let simCtx;
	let display;
	let simRunning = false;
	let iterationCount = 0;

	let simWidth = 100;
	let simHeight = 100;
	let simAlpha = 1.0;
	let simBeta = 0.4;
	let simGamma = 0.0001;
	
	onMount(() => {
		// Start render loop
		parseURLParams();
		initSim();
		simulationLoop();
	});

	function simulationLoop() {
		display.renderFrame();
		stepSim();
		if (simRunning) {
			requestAnimationFrame(simulationLoop);
		}
	}

	function stepSim() {
		simCtx.step_simulation();
		simCtx.update_vertex_colors();
		display.updateColorBuffer(simCtx.get_vertex_colors());
		iterationCount += 1;
	}

	function toggleSim() {
		simRunning = !simRunning;
		if (simRunning) {
			requestAnimationFrame(simulationLoop);
		}
	}

	function initSim() {
		simCtx = snowflakeSimLib.SnowflakeSimContext.new(simWidth, simHeight, simAlpha, simBeta, simGamma);
		simCtx.set_cell(simWidth / 2 + 1, simHeight / 2, 1.0);
		simCtx.step_simulation();
		simCtx.create_vertex_positions();
		simCtx.update_vertex_colors();
		display.setSimSize(simWidth, simHeight);
		display.updatePositionBuffer(simCtx.get_vertex_positions());
		display.updateColorBuffer(simCtx.get_vertex_colors());
	}

	function timeSim() {
		console.log("Starting simulation timing");
		let startTime = performance.now();

		for (let i = 0; i < 1000; i++) {
			simCtx.step_simulation();
		}

		let endTime = performance.now();
		let elapsedTime = endTime - startTime
		console.log(`Simulation took ${elapsedTime.toFixed(2)} ms (${(elapsedTime / 1000).toFixed(2)} ms)`);
	}

	function parseURLParams() {
		const urlParams = new URLSearchParams(location.search);
		if (urlParams.get("size") != null) {
			let size = urlParams.get("size").split("x");
			simWidth = size[0];
			simHeight = size[1];
		}	
	}

	$: if (simCtx) simCtx.set_alpha(simAlpha);
	$: if (simCtx) simCtx.set_beta(simBeta);
	$: if (simCtx) simCtx.set_gamma(simGamma);
	
</script>

<svelte:head>
	<title>Snowflake Simulation</title>
</svelte:head>

<main >
	<div id="column">
		<div id="siminfo">
			<div>
				Iteration: {iterationCount} 
			</div>
			<div>
				<nobr>
					α: <input type="number" bind:value={simAlpha}>    
					β: <input type="number" bind:value={simBeta}>
					γ: <input type="number" bind:value={simGamma}>
				</nobr>
			</div>
		</div>
		<Display bind:this={display}></Display>
		<div>
		<button on:click={toggleSim}>
			{#if !simRunning}
				<Fa icon={faPlay} size="1.5x" color="white" />
			{:else}
				<Fa icon={faPause} size="1.5x" color="white" />
			{/if}
		</button>
		<button on:click={display.screenshot()}>
			<Fa icon={faDownload} size="1.5x" color="white" />
		</button>
		</div>
	</div>
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
		all: unset;
		padding: 0.5em;
	}

	input[type='number'] {
		all:unset;
		border: 1px;
		border-style: solid;
		border-radius: 8px;
		border-color: grey;
		padding: 0.15em;
		padding-left: 0.5em;
		padding-right: 0.5em;
		width: 55px;
		-moz-appearance:textfield;
	}

	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
		-webkit-appearance: none;
	}

	#column {
		width: 50%;
		height: 100%;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 10px;
	}

	#siminfo {
		white-space: pre;
        font-family: monospace;
        padding-top: 0;
        display: block;
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

</style>