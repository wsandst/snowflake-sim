
<script>
    // Display/render canvas component for viewing a Snowflake Simulation
    import { onMount } from 'svelte';
    import * as render from './render'

    // Import shader sources as strings
    import vertShaderSource from './shaders/vertex.vert'
    import fragShaderSource from './shaders/fragment.frag'
import { sqrDist } from 'gl-matrix/cjs/vec3';

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
	let offset = [0, 0];
	let scale = 1.0;

	onMount(() => {
		canvas.width = 500;
		canvas.height = 500;
		glCtx = canvas.getContext("webgl", {antialias: true});
        
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
        render.updateBufferData(glCtx, buffers.color, buffer);
    }

	export function setSimSize(width, height) {
		hexWidth = width;
		hexHeight = height;
		scale = 490/(width*Math.sqrt(3));
		offset = [5, 35];
	}

	export function renderFrame() {
		// call again next time we can draw
		render.draw(glCtx, programInfo, buffers, vertexCount, offset, scale, color);
	}

	function zoom(event) {
		let x = event.offsetX / scale + offset[0];
		let y = (500 - event.offsetY) / scale + offset[1];
		let zoomScale = (event.deltaY * -0.001);
		let newScale = scale + scale*zoomScale;
		let scaleDelta = newScale - scale;
		let offsetX = -(x * scaleDelta);
		let offsetY = -(y * scaleDelta);
		scale = newScale;
		offset = [offsetX, offsetY];
		renderFrame();

		/*let x = (event.offsetX - offset[0]) / scale;
		let y = (event.offsetY - offset[1]) / scale;
		let zoomScale = (event.deltaY * -0.001);
		scale = scale + scale*zoomScale;
		//let scaleDelta = newScale - scale;
		//let offsetX = (x * scaleDelta);
		//let offsetY = (y * scaleDelta);
		//console.log(offsetX, offsetY, offset);
		//scale = newScale;
		offset = [event.offsetX - x * scale, event.offsetY - y * scale];
		renderFrame();*/
	}

</script>

<canvas bind:this={canvas} on:mousewheel={zoom}>

</canvas>

<style>
    canvas {
		width: 500px;
		height: 500px;
	}
</style>