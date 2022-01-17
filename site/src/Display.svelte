
<script>
    // Display/render canvas component for viewing a Snowflake Simulation
    import { onMount } from 'svelte';
    import * as render from './render'

    // Import shader sources as strings
    import vertShaderSource from './shaders/vertex.vert'
    import fragShaderSource from './shaders/fragment.frag'

	let canvas;
	// WebGL internal state
	let glCtx;
	let shaderProgram;
	let buffers;
	let programInfo;
	let vertexCount = 0;
	// Drawing settings
	let hexWidth;
	let hexHeight
	const color = [0.5, 0.82, 0.96, 1.0];
	const scale = 2.75; //0.055 * (50 / hexWidth), [-2.35, -2, -6.0];
	const offset = [-2.35, -2, -6.0]

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
                vertexColor: glCtx.getAttribLocation(shaderProgram, 'aVertexColor'),
			},
				uniformLocations: {
				projectionMatrix: glCtx.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
				modelViewMatrix: glCtx.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
				hexColor: glCtx.getUniformLocation(shaderProgram, 'hexColor'),
			},
		};

		buffers = render.initBuffers(glCtx);

	});

    /**
     * Update the vertex position buffer
     * @param buffer vertex position buffer (x, y) * N
     */
    export function updatePositionBuffer(buffer) {
        render.updateBufferData(glCtx, buffers.position, buffer);
        vertexCount = buffer.length / 2;
    }

    /**
     * Update the vertex color buffer
     * @param buffer vertex color buffer (r, g, b, a) * N
     */
    export function updateColorBuffer(buffer) {
		console.log(buffer);
        render.updateBufferData(glCtx, buffers.color, buffer);
    }

	export function setSimSize(width, height) {
		hexWidth = width;
		hexHeight = height;
	}

	export function renderFrame() {
		// call again next time we can draw
		render.draw(glCtx, programInfo, buffers, vertexCount, scale / hexWidth, offset, color);
	}

</script>

<canvas bind:this={canvas}>

</canvas>

<style>
    
</style>