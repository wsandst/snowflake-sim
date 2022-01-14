import App from './App.svelte';
import wasm from '../../Cargo.toml';

const init = async() => {
	const snowflakeSimLib = await wasm();

	const app = new App({
		target: document.body,
		props: {
			snowflakeSimLib: snowflakeSimLib
		}
	});
};

init();