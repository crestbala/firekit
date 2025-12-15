export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png","icons/apple.svg","icons/linux.svg","icons/loading.svg","icons/windows.svg","images/diagram.webp"]),
	mimeTypes: {".png":"image/png",".svg":"image/svg+xml",".webp":"image/webp"},
	_: {
		client: {start:"_app/immutable/entry/start.Bq_uAjBV.js",app:"_app/immutable/entry/app.xkkqkZ4D.js",imports:["_app/immutable/entry/start.Bq_uAjBV.js","_app/immutable/chunks/DNLHdk0b.js","_app/immutable/entry/app.xkkqkZ4D.js","_app/immutable/chunks/DNLHdk0b.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/5.js'))
		],
		routes: [
			{
				id: "/pokemon/[slug]",
				pattern: /^\/pokemon\/([^/]+?)\/?$/,
				params: [{"name":"slug","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		prerendered_routes: new Set(["/","/infos","/pokemon"]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
