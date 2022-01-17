<script>
	import Display from './Display.svelte'
	import { onMount } from 'svelte';
import { xlink_attr } from 'svelte/internal';

	export let snowflakeSimLib;
	let simCtx;
	let display;
	let simRunning = false;
	let iterationCount = 0;
	const simSize = 100;
	
	onMount(() => {
		// Start render loop
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
		console.log(`Iteration ${iterationCount}`);
	}

	function toggleSim() {
		simRunning = !simRunning;
		if (simRunning) {
			requestAnimationFrame(simulationLoop);
		}
	}

	function initSim() {
		simCtx = snowflakeSimLib.SnowflakeSimContext.new(simSize, simSize, 1.0, 0.4, 0.0001);
		simCtx.set_cell(simSize / 2, simSize / 2, 1.0);
		simCtx.step_simulation();
		simCtx.create_vertex_positions();
		simCtx.update_vertex_colors();
		display.setSimSize(simSize, simSize);
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
	
</script>

<svelte:head>
	<title>Snowflake Simulation</title>
</svelte:head>

<main >
	<Display bind:this={display}></Display>
	<button on:click={toggleSim}>
		{#if !simRunning}
			Start Simulation
		{:else}
			Stop Simulation
		{/if}
	</button>
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
		width: 120px;
		height: 30px;
	}

	canvas {
		width : 500;
		height : 500;
	}
</style>