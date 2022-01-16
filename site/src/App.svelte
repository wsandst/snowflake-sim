<script>
	import Display from './Display.svelte'
	import { onMount } from 'svelte';

	export let snowflakeSimLib;
	let simCtx;
	let display;
	
	onMount(() => {
		// Start render loop
		initSim();
		simulationLoop();
	});

	function simulationLoop() {
		display.renderFrame();
		requestAnimationFrame(simulationLoop);
	}

	function stepSim() {

	}

	function initSim() {
		simCtx = snowflakeSimLib.SnowflakeSimContext.new(500, 500, 1.0, 0.4, 0.0001);
		display.updatePositionBuffer([
            1.0,  1.0,
            -1.0,  1.0,
            1.0, -1.0,
            -1.0, -1.0]
        );
		display.updateColorBuffer([
            1.0,  1.0,  1.0,  1.0,    // white
            1.0,  0.0,  0.0,  1.0,    // red
            0.0,  1.0,  0.0,  1.0,    // green
            0.0,  0.0,  1.0,  1.0,    // blue
        ]);
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
	<button on:click={stepSim}>
		Step
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
		width: 70px;
		height: 30px;
	}

	canvas {
		width : 500;
		height : 500;
	}
</style>