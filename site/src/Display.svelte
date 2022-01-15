
<script>
    // Display/render canvas component for viewing a Snowflake Simulation
    import { onMount } from 'svelte';
    import * as render from './render'

    // Import shader sources as strings
    import vertShaderSource from './shaders/vertex.vert'
    import fragShaderSource from './shaders/fragment.frag'

	let canvas;
	let glCtx;
	let shaderProgram;
	let buffers;
	let programInfo;
	let rotateX = 0;

	onMount(() => {
		canvas.width = 500;
		canvas.height = 500;
		glCtx = canvas.getContext("webgl");
        
        // Setup WebGL context
		shaderProgram = render.initShaderProgram(glCtx, vertShaderSource, fragShaderSource);

		programInfo = {
				program: shaderProgram,
				attribLocations: {
				vertexPosition: glCtx.getAttribLocation(shaderProgram, 'aVertexPosition'),
			},
				uniformLocations: {
				projectionMatrix: glCtx.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
				modelViewMatrix: glCtx.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
			},
		};

		buffers = render.initBuffers(glCtx);
        // Start render loop
		renderFrame();
	});

	function renderFrame() {
		// call again next time we can draw
		requestAnimationFrame(renderFrame);
		rotateX += 0.01;
		render.draw(glCtx, programInfo, buffers, rotateX);
	}

</script>

<canvas bind:this={canvas}>

</canvas>

<style>
    
</style>