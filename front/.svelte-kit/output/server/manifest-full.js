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
		client: {start:"_app/immutable/entry/start.OtF0keKS.js",app:"_app/immutable/entry/app.DSR5Iml2.js",imports:["_app/immutable/entry/start.OtF0keKS.js","_app/immutable/chunks/C1W_HtSc.js","_app/immutable/entry/app.DSR5Iml2.js","_app/immutable/chunks/C1W_HtSc.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js')),
			__memo(() => import('./nodes/3.js')),
			__memo(() => import('./nodes/4.js')),
			__memo(() => import('./nodes/5.js'))
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			},
			{
				id: "/infos",
				pattern: /^\/infos\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			},
			{
				id: "/pokemon",
				pattern: /^\/pokemon\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 4 },
				endpoint: null
			},
			{
				id: "/pokemon/[slug]",
				pattern: /^\/pokemon\/([^/]+?)\/?$/,
				params: [{"name":"slug","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,], errors: [1,], leaf: 5 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
