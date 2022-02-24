<script>
	/*
		Todo:
			Fix zoom, add either panning or allow for adding of ice by mouse
			Fix issues with preset URL sometimes not working
			Add functionality for generating random starting clusters
	*/
	import Display from './Display.svelte'
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa'
	import { faPause, faPlay, faDownload, faUndo, faBolt, faShare } from '@fortawesome/free-solid-svg-icons'
	
	export let snowflakeSimLib;
	let simCtx;
	let display;
	let simRunning = false;
	let simSpeedup = false;
	let runningPlayback = false;
	let iterationCount = 0;

	let simWidth = 100;
	let simHeight = 100;
	let simAlpha = 1.0;
	let simBeta = 0.4;
	let simGamma = 0.0001;
	let simAlphaRand = 0.0;
	let simBetaRand = 0.0;
	let simGammaRand = 0.0;
	let simRandSeed = BigInt(34917983469832);
	let simPreset = null;
	let simTransparentBackground = false;
	
	onMount(() => {
		// Start render loop
		parseURLParams();
		initSim();
		//timeSim();
	});

	function simulationLoop() {
		if (iterationCount == 0) {
			simCtx.init_tracking();
		}
		if (!simSpeedup) {
			stepSim();
		}
		else {
			// Speedup, run as much as possible for one frame
			const start = performance.now();
			let delta = 0;
			while (delta <= (1.0/60.0)*1000) {
				stepSim();
				delta = (performance.now() - start);
			}
		}
		display.renderFrame();
		if (simRunning) {
			requestAnimationFrame(simulationLoop);
		}
	}

	function stepSim() {
		if (!runningPlayback) {
			simCtx.step_simulation();
		}
		else {
			simCtx.step_simulation_playback();
			updateSimParams();
		}
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

	function toggleSpeedup() {
		simSpeedup = !simSpeedup;
	}

	function initSim() {
		simRunning = false;
		simCtx = snowflakeSimLib.SnowflakeSimContext.new(simWidth, simHeight, simAlpha, simBeta, simGamma);
		simCtx.set_random_seed(simRandSeed);
		simCtx.set_cell(simWidth / 2 + 1, simHeight / 2, 1.0);
		simCtx.set_alpha_rand(0.3);
		simCtx.set_transparent_background(simTransparentBackground);
		simCtx.create_vertex_positions();
		simCtx.update_vertex_colors();
		display.setSimSize(simWidth, simHeight);
		display.updatePositionBuffer(simCtx.get_vertex_positions());
		display.updateColorBuffer(simCtx.get_vertex_colors());
		if (runningPlayback) {
			simCtx.init_playback(simPreset);
			updateSimParams();
		}
		display.renderFrame();
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

	function updateSimParams() {
		simAlpha = simCtx.get_alpha();
		simBeta = simCtx.get_beta();
		simGamma = simCtx.get_gamma();
		simAlphaRand = simCtx.get_alpha_rand();
	}

	function copySimReprToClipboard() {
		if (!simCtx) {
			return;
		}
		if (iterationCount == 0) {
			simCtx.init_tracking();
		}
		var text = simCtx.get_simulation_string_repr();
		let url = location.href;
		if (url.includes("?")) {
			url = url + "&preset=" + text;
		}
		else {
			url = url + "?preset=" + text;
		}
		console.log("Simulation url: "+ url);
		navigator.clipboard.writeText(url).then(function() {
			console.log('Copied simulation URL to clipboard');
		}, function(err) {
			console.error('Could not copy text: ', err);
		});
	}

	function parseURLParams() {
		const urlParams = new URLSearchParams(location.search);
		if (urlParams.get("size") != null) {
			let size = urlParams.get("size").split("x");
			simWidth = size[0];
			simHeight = size[1];
		}	
		if (urlParams.get("seed") != null) {
			simRandSeed = urlParams.get("seed");
		}	
		if (urlParams.get("preset") != null) {
			simPreset = urlParams.get("preset");
			runningPlayback = true;
		}
		if (urlParams.get("color") != null) {
			let color = urlParams.get("color");
			display.setSimColor(color);
		}
		if (urlParams.get("transparent") != null) {
			simTransparentBackground = true;
		}
	}

	$: if (simCtx) simCtx.set_alpha(simAlpha);
	$: if (simCtx) simCtx.set_beta(simBeta);
	$: if (simCtx) simCtx.set_gamma(simGamma);
	$: if (simCtx) simCtx.set_alpha_rand(simAlphaRand);
	$: if (simCtx) simCtx.set_beta_rand(simBetaRand);
	$: if (simCtx) simCtx.set_gamma_rand(simGammaRand);
	
</script>

<svelte:head>
	<title>Snowflake Simulation</title>
</svelte:head>

<main >
	<div id="column">
		<div id="siminfo">
			<div style="margin-bottom: 0.3em">
				Iteration: {iterationCount} 
			</div>
			<div id="paraminputs">
<p>&nbsp;α:</p> <input type="number" bind:value={simAlpha} disabled={runningPlayback} title="Alpha (Vapor Addition) parameter">    
<p>&nbsp;β:</p> <input type="number" bind:value={simBeta} disabled={runningPlayback} title="Beta (Background Vapor) parameter">
<p>&nbsp;γ:</p> <input type="number" bind:value={simGamma} disabled={runningPlayback} title="Gamma (Vapor Diffusion) parameter">
<p>αr:</p> <input type="number" bind:value={simAlphaRand} disabled={runningPlayback} title="Alpha (Vapor Addition) randomization parameter, in percent">   
			</div>
		</div>
		<Display bind:this={display}></Display>
		<div id="controls">
			<button on:click={toggleSim} title={!simRunning ? "Start Simulation" : "Pause Simulation"}>
				<Fa icon={!simRunning ? faPlay : faPause } size="1.5x" color="white" />
			</button>
			<button on:click={toggleSpeedup} title={!simSpeedup ? "Speedup Simulation" : "Slow down Simulation"}>
				<Fa icon={faBolt} size="1.5x" color={simSpeedup ? "white" : "grey"} />
			</button>
			<button on:click={initSim} title="Reset Simulation">
				<Fa icon={faUndo} size="1.5x" color="white"/>
			</button>
			<button on:click={copySimReprToClipboard} title="Share Simulation by URL">
				<Fa icon={faShare} size="1.5x" color="white" />
			</button>
			<button on:click={display.screenshot()} title="Download image of Simulation">
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

	input:disabled {
		color:grey;
	}

	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
		-webkit-appearance: none;
	}

	#column {
		height: 100%;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 5px;
	}

	#siminfo {
		white-space: pre;
        font-family: monospace;
        padding-top: 0;
        display: block;
		text-align: center;
		display: flex;
		flex-direction: column;
	}

	#paraminputs {
		display: flex;  
		flex-direction: row;
		align-items: center;
		justify-content: center;
		align-content: center;
		text-align: center;
		flex-wrap: wrap;
	}

	#paraminputs p {
		margin-left: 1em;
		margin-right: 0.3em;
		margin-top: 0.6em;
		margin-bottom: 0.6em;
	}

	#controls {
		display: flex;
		flex-direction: row;
		justify-content: flex-start;
		width: 100%;
		margin-left: 1.5em;
	}

	@media only screen and (max-width: 480px) {
        #column {
            width: 340px;
        }
		#paraminputs {
			width: 230px;
		}
    }

	@media only screen and (min-width: 480px) {
        #column {
            width: 500px;
        }
    }

	@media only screen and (min-width: 1020px) {
        #column {
            width: 650px;
        }
    }
</style>