import { server$ as server$ } from "solid-start/server";
import { createOpenGraphImage as createOpenGraphImage$ } from "@solid-mediakit/open-graph";
const img = server$(() => {
	return createOpenGraphImage$(<div>123</div>);
});
const coolVar = <img src={img.src} />;
