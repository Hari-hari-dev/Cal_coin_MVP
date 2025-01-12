(this["webpackJsonpcivic-react"]=this["webpackJsonpcivic-react"]||[]).push([[3],{188:function(e,t,n){"use strict";var i=Object.prototype.hasOwnProperty,r="~";function o(){}function s(e,t,n){this.fn=e,this.context=t,this.once=n||!1}function a(e,t,n,i,o){if("function"!==typeof n)throw new TypeError("The listener must be a function");var a=new s(n,i||e,o),c=r?r+t:t;return e._events[c]?e._events[c].fn?e._events[c]=[e._events[c],a]:e._events[c].push(a):(e._events[c]=a,e._eventsCount++),e}function c(e,t){0===--e._eventsCount?e._events=new o:delete e._events[t]}function l(){this._events=new o,this._eventsCount=0}Object.create&&(o.prototype=Object.create(null),(new o).__proto__||(r=!1)),l.prototype.eventNames=function(){var e,t,n=[];if(0===this._eventsCount)return n;for(t in e=this._events)i.call(e,t)&&n.push(r?t.slice(1):t);return Object.getOwnPropertySymbols?n.concat(Object.getOwnPropertySymbols(e)):n},l.prototype.listeners=function(e){var t=r?r+e:e,n=this._events[t];if(!n)return[];if(n.fn)return[n.fn];for(var i=0,o=n.length,s=new Array(o);i<o;i++)s[i]=n[i].fn;return s},l.prototype.listenerCount=function(e){var t=r?r+e:e,n=this._events[t];return n?n.fn?1:n.length:0},l.prototype.emit=function(e,t,n,i,o,s){var a=r?r+e:e;if(!this._events[a])return!1;var c,l,h=this._events[a],d=arguments.length;if(h.fn){switch(h.once&&this.removeListener(e,h.fn,void 0,!0),d){case 1:return h.fn.call(h.context),!0;case 2:return h.fn.call(h.context,t),!0;case 3:return h.fn.call(h.context,t,n),!0;case 4:return h.fn.call(h.context,t,n,i),!0;case 5:return h.fn.call(h.context,t,n,i,o),!0;case 6:return h.fn.call(h.context,t,n,i,o,s),!0}for(l=1,c=new Array(d-1);l<d;l++)c[l-1]=arguments[l];h.fn.apply(h.context,c)}else{var u,f=h.length;for(l=0;l<f;l++)switch(h[l].once&&this.removeListener(e,h[l].fn,void 0,!0),d){case 1:h[l].fn.call(h[l].context);break;case 2:h[l].fn.call(h[l].context,t);break;case 3:h[l].fn.call(h[l].context,t,n);break;case 4:h[l].fn.call(h[l].context,t,n,i);break;default:if(!c)for(u=1,c=new Array(d-1);u<d;u++)c[u-1]=arguments[u];h[l].fn.apply(h[l].context,c)}}return!0},l.prototype.on=function(e,t,n){return a(this,e,t,n,!1)},l.prototype.once=function(e,t,n){return a(this,e,t,n,!0)},l.prototype.removeListener=function(e,t,n,i){var o=r?r+e:e;if(!this._events[o])return this;if(!t)return c(this,o),this;var s=this._events[o];if(s.fn)s.fn!==t||i&&!s.once||n&&s.context!==n||c(this,o);else{for(var a=0,l=[],h=s.length;a<h;a++)(s[a].fn!==t||i&&!s[a].once||n&&s[a].context!==n)&&l.push(s[a]);l.length?this._events[o]=1===l.length?l[0]:l:c(this,o)}return this},l.prototype.removeAllListeners=function(e){var t;return e?(t=r?r+e:e,this._events[t]&&c(this,t)):(this._events=new o,this._eventsCount=0),this},l.prototype.off=l.prototype.removeListener,l.prototype.addListener=l.prototype.on,l.prefixed=r,l.EventEmitter=l,e.exports=l},189:function(e,t,n){const i=n(190);e.exports=i("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz")},190:function(e,t,n){"use strict";e.exports=function(e){if(e.length>=255)throw new TypeError("Alphabet too long");for(var t=new Uint8Array(256),n=0;n<t.length;n++)t[n]=255;for(var i=0;i<e.length;i++){var r=e.charAt(i),o=r.charCodeAt(0);if(255!==t[o])throw new TypeError(r+" is ambiguous");t[o]=i}var s=e.length,a=e.charAt(0),c=Math.log(s)/Math.log(256),l=Math.log(256)/Math.log(s);function h(e){if("string"!==typeof e)throw new TypeError("Expected String");if(0===e.length)return new Uint8Array;for(var n=0,i=0,r=0;e[n]===a;)i++,n++;for(var o=(e.length-n)*c+1>>>0,l=new Uint8Array(o);e[n];){var h=t[e.charCodeAt(n)];if(255===h)return;for(var d=0,u=o-1;(0!==h||d<r)&&-1!==u;u--,d++)h+=s*l[u]>>>0,l[u]=h%256>>>0,h=h/256>>>0;if(0!==h)throw new Error("Non-zero carry");r=d,n++}for(var f=o-r;f!==o&&0===l[f];)f++;for(var v=new Uint8Array(i+(o-f)),p=i;f!==o;)v[p++]=l[f++];return v}return{encode:function(t){if(t instanceof Uint8Array||(ArrayBuffer.isView(t)?t=new Uint8Array(t.buffer,t.byteOffset,t.byteLength):Array.isArray(t)&&(t=Uint8Array.from(t))),!(t instanceof Uint8Array))throw new TypeError("Expected Uint8Array");if(0===t.length)return"";for(var n=0,i=0,r=0,o=t.length;r!==o&&0===t[r];)r++,n++;for(var c=(o-r)*l+1>>>0,h=new Uint8Array(c);r!==o;){for(var d=t[r],u=0,f=c-1;(0!==d||u<i)&&-1!==f;f--,u++)d+=256*h[f]>>>0,h[f]=d%s>>>0,d=d/s>>>0;if(0!==d)throw new Error("Non-zero carry");i=u,r++}for(var v=c-i;v!==c&&0===h[v];)v++;for(var p=a.repeat(n);v<c;++v)p+=e.charAt(h[v]);return p},decodeUnsafe:h,decode:function(e){var t=h(e);if(t)return t;throw new Error("Non-base"+s+" character")}}}},191:function(e,t,n){"use strict";(function(e){function i(e){return void 0===e.version}function r(e){return i(e)?e.serialize({verifySignatures:!1,requireAllSignatures:!1}):e.serialize()}n.d(t,"a",(function(){return i})),n.d(t,"b",(function(){return r}))}).call(this,n(4).Buffer)},195:function(e,t,n){"use strict";n.r(t),n.d(t,"StandardSolflareMetaMaskWalletAccount",(function(){return I}));var i=n(6),r=n(188),o=n.n(r),s=n(189),a=n.n(s);var c={randomUUID:"undefined"!==typeof crypto&&crypto.randomUUID&&crypto.randomUUID.bind(crypto)};let l;const h=new Uint8Array(16);function d(){if(!l&&(l="undefined"!==typeof crypto&&crypto.getRandomValues&&crypto.getRandomValues.bind(crypto),!l))throw new Error("crypto.getRandomValues() not supported. See https://github.com/uuidjs/uuid#getrandomvalues-not-supported");return l(h)}const u=[];for(let z=0;z<256;++z)u.push((z+256).toString(16).slice(1));function f(e){let t=arguments.length>1&&void 0!==arguments[1]?arguments[1]:0;return u[e[t+0]]+u[e[t+1]]+u[e[t+2]]+u[e[t+3]]+"-"+u[e[t+4]]+u[e[t+5]]+"-"+u[e[t+6]]+u[e[t+7]]+"-"+u[e[t+8]]+u[e[t+9]]+"-"+u[e[t+10]]+u[e[t+11]]+u[e[t+12]]+u[e[t+13]]+u[e[t+14]]+u[e[t+15]]}var v=function(e,t,n){if(c.randomUUID&&!t&&!e)return c.randomUUID();const i=(e=e||{}).random||(e.rng||d)();if(i[6]=15&i[6]|64,i[8]=63&i[8]|128,t){n=n||0;for(let e=0;e<16;++e)t[n+e]=i[e];return t}return f(i)},p=n(191),m=function(e,t,n,i){return new(n||(n=Promise))((function(r,o){function s(e){try{c(i.next(e))}catch(t){o(t)}}function a(e){try{c(i.throw(e))}catch(t){o(t)}}function c(e){var t;e.done?r(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(s,a)}c((i=i.apply(e,t||[])).next())}))};function g(e){return m(this,void 0,void 0,(function*(){try{return yield e.request({method:"wallet_getSnaps"}),!0}catch(t){return!1}}))}var y=n(177),_=n(178),w=n(179);const b=["solana:mainnet","solana:devnet","solana:testnet","solana:localnet"];function E(e){return b.includes(e)}var A,x,M,T,S,j,C=function(e,t,n,i){if("a"===n&&!i)throw new TypeError("Private accessor was defined without a getter");if("function"===typeof t?e!==t||!i:!t.has(e))throw new TypeError("Cannot read private member from an object whose class did not declare it");return"m"===n?i:"a"===n?i.call(e):i?i.value:t.get(e)},U=function(e,t,n,i,r){if("m"===i)throw new TypeError("Private method is not writable");if("a"===i&&!r)throw new TypeError("Private accessor was defined without a setter");if("function"===typeof t?e!==t||!r:!t.has(e))throw new TypeError("Cannot write private member to an object whose class did not declare it");return"a"===i?r.call(e,n):r?r.value=n:t.set(e,n),n};const k=b,K=[y.a,_.a,w.a];class I{get address(){return C(this,A,"f")}get publicKey(){return C(this,x,"f").slice()}get chains(){return C(this,M,"f").slice()}get features(){return C(this,T,"f").slice()}get label(){return C(this,S,"f")}get icon(){return C(this,j,"f")}constructor(e){let{address:t,publicKey:n,label:i,icon:r}=e;A.set(this,void 0),x.set(this,void 0),M.set(this,void 0),T.set(this,void 0),S.set(this,void 0),j.set(this,void 0),new.target===I&&Object.freeze(this),U(this,A,t,"f"),U(this,x,n,"f"),U(this,M,k,"f"),U(this,T,K,"f"),U(this,S,i,"f"),U(this,j,r,"f")}}A=new WeakMap,x=new WeakMap,M=new WeakMap,T=new WeakMap,S=new WeakMap,j=new WeakMap;var O=function(e,t,n,i){return new(n||(n=Promise))((function(r,o){function s(e){try{c(i.next(e))}catch(t){o(t)}}function a(e){try{c(i.throw(e))}catch(t){o(t)}}function c(e){var t;e.done?r(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(s,a)}c((i=i.apply(e,t||[])).next())}))};class P extends o.a{constructor(e){super(),this._network="mainnet-beta",this._iframeParams={},this._element=null,this._iframe=null,this._publicKey=null,this._account=null,this._isConnected=!1,this._connectHandler=null,this._messageHandlers={},this._handleEvent=e=>{var t,n;switch(e.type){case"connect":return this._collapseIframe(),void((null===(t=e.data)||void 0===t?void 0:t.publicKey)?(this._publicKey=e.data.publicKey,this._isConnected=!0,this._connectHandler&&(this._connectHandler.resolve(),this._connectHandler=null),this._connected()):(this._connectHandler&&(this._connectHandler.reject(),this._connectHandler=null),this._disconnected()));case"disconnect":return this._connectHandler&&(this._connectHandler.reject(),this._connectHandler=null),void this._disconnected();case"accountChanged":return void((null===(n=e.data)||void 0===n?void 0:n.publicKey)?(this._publicKey=e.data.publicKey,this.emit("accountChanged",this.publicKey),this._standardConnected()):(this.emit("accountChanged",void 0),this._standardDisconnected()));default:return}},this._handleResize=e=>{"full"===e.resizeMode?"fullscreen"===e.params.mode?this._expandIframe():"hide"===e.params.mode&&this._collapseIframe():"coordinates"===e.resizeMode&&this._resizeIframe(e.params)},this._handleMessage=e=>{var t;if("solflareIframeToWalletAdapter"!==(null===(t=e.data)||void 0===t?void 0:t.channel))return;const n=e.data.data||{};if("event"===n.type)this._handleEvent(n.event);else if("resize"===n.type)this._handleResize(n);else if("response"===n.type&&this._messageHandlers[n.id]){const{resolve:e,reject:t}=this._messageHandlers[n.id];delete this._messageHandlers[n.id],n.error?t(n.error):e(n.result)}},this._removeElement=()=>{this._element&&(this._element.remove(),this._element=null)},this._removeDanglingElements=()=>{const e=document.getElementsByClassName("solflare-metamask-wallet-adapter-iframe");for(const t of e)t.parentElement&&t.remove()},this._injectElement=()=>{this._removeElement(),this._removeDanglingElements();const e=Object.assign(Object.assign({},this._iframeParams),{mm:!0,v:1,cluster:this._network||"mainnet-beta",origin:window.location.origin||"",title:document.title||""}),t=Object.keys(e).map((t=>"".concat(t,"=").concat(encodeURIComponent(e[t])))).join("&"),n="".concat(P.IFRAME_URL,"?").concat(t);this._element=document.createElement("div"),this._element.className="solflare-metamask-wallet-adapter-iframe",this._element.innerHTML="\n      <iframe src='".concat(n,"' style='position: fixed; top: 0; bottom: 0; left: 0; right: 0; width: 100%; height: 100%; border: none; border-radius: 0; z-index: 99999; color-scheme: auto;' allowtransparency='true'></iframe>\n    "),document.body.appendChild(this._element),this._iframe=this._element.querySelector("iframe"),window.addEventListener("message",this._handleMessage,!1)},this._collapseIframe=()=>{this._iframe&&(this._iframe.style.top="",this._iframe.style.right="",this._iframe.style.height="2px",this._iframe.style.width="2px")},this._expandIframe=()=>{this._iframe&&(this._iframe.style.top="0px",this._iframe.style.bottom="0px",this._iframe.style.left="0px",this._iframe.style.right="0px",this._iframe.style.width="100%",this._iframe.style.height="100%")},this._resizeIframe=e=>{this._iframe&&(this._iframe.style.top=isFinite(e.top)?"".concat(e.top,"px"):"",this._iframe.style.bottom=isFinite(e.bottom)?"".concat(e.bottom,"px"):"",this._iframe.style.left=isFinite(e.left)?"".concat(e.left,"px"):"",this._iframe.style.right=isFinite(e.right)?"".concat(e.right,"px"):"",this._iframe.style.width=isFinite(e.width)?"".concat(e.width,"px"):e.width,this._iframe.style.height=isFinite(e.height)?"".concat(e.height,"px"):e.height)},this._sendIframeMessage=e=>{if(!this.connected||!this.publicKey)throw new Error("Wallet not connected");return new Promise(((t,n)=>{var i,r;const o=v();this._messageHandlers[o]={resolve:t,reject:n},null===(r=null===(i=this._iframe)||void 0===i?void 0:i.contentWindow)||void 0===r||r.postMessage({channel:"solflareWalletAdapterToIframe",data:Object.assign({id:o},e)},"*")}))},this._connected=()=>{this._isConnected=!0,this.emit("connect",this.publicKey),this._standardConnected()},this._disconnected=()=>{this._publicKey=null,this._isConnected=!1,window.removeEventListener("message",this._handleMessage,!1),this._removeElement(),this.emit("disconnect"),this._standardDisconnected()},this._standardConnected=()=>{if(!this.publicKey)return;const e=this.publicKey.toString();this._account&&this._account.address===e||(this._account=new I({address:e,publicKey:this.publicKey.toBytes()}),this.emit("standard_change",{accounts:this.standardAccounts}))},this._standardDisconnected=()=>{this._account&&(this._account=null,this.emit("standard_change",{accounts:this.standardAccounts}))},(null===e||void 0===e?void 0:e.network)&&(this._network=null===e||void 0===e?void 0:e.network),window.SolflareMetaMaskParams&&(this._iframeParams=Object.assign(Object.assign({},this._iframeParams),window.SolflareMetaMaskParams)),(null===e||void 0===e?void 0:e.params)&&(this._iframeParams=Object.assign(Object.assign({},this._iframeParams),null===e||void 0===e?void 0:e.params))}get publicKey(){return this._publicKey?new i.PublicKey(this._publicKey):null}get standardAccount(){return this._account}get standardAccounts(){return this._account?[this._account]:[]}get isConnected(){return this._isConnected}get connected(){return this.isConnected}get autoApprove(){return!1}connect(){return O(this,void 0,void 0,(function*(){this.connected||(this._injectElement(),yield new Promise(((e,t)=>{this._connectHandler={resolve:e,reject:t}})))}))}disconnect(){return O(this,void 0,void 0,(function*(){yield this._sendIframeMessage({method:"disconnect"}),this._disconnected()}))}signTransaction(e){var t;return O(this,void 0,void 0,(function*(){if(!this.connected||!this.publicKey)throw new Error("Wallet not connected");try{const t=Object(p.b)(e),n=yield this._sendIframeMessage({method:"signTransactionV2",params:{transaction:a.a.encode(t)}}),{transaction:r}=n;return Object(p.a)(e)?i.Transaction.from(a.a.decode(r)):i.VersionedTransaction.deserialize(a.a.decode(r))}catch(n){throw new Error((null===(t=null===n||void 0===n?void 0:n.toString)||void 0===t?void 0:t.call(n))||"Failed to sign transaction")}}))}signAllTransactions(e){var t;return O(this,void 0,void 0,(function*(){if(!this.connected||!this.publicKey)throw new Error("Wallet not connected");try{const t=e.map((e=>Object(p.b)(e))),{transactions:n}=yield this._sendIframeMessage({method:"signAllTransactionsV2",params:{transactions:t.map((e=>a.a.encode(e)))}});return n.map(((t,n)=>Object(p.a)(e[n])?i.Transaction.from(a.a.decode(t)):i.VersionedTransaction.deserialize(a.a.decode(t))))}catch(n){throw new Error((null===(t=null===n||void 0===n?void 0:n.toString)||void 0===t?void 0:t.call(n))||"Failed to sign transactions")}}))}signAndSendTransaction(e,t){var n;return O(this,void 0,void 0,(function*(){if(!this.connected||!this.publicKey)throw new Error("Wallet not connected");try{const n=Object(p.b)(e),{signature:i}=yield this._sendIframeMessage({method:"signAndSendTransaction",params:{transaction:a.a.encode(n),options:t}});return i}catch(i){throw new Error((null===(n=null===i||void 0===i?void 0:i.toString)||void 0===n?void 0:n.call(i))||"Failed to sign and send transaction")}}))}signMessage(e){let t=arguments.length>1&&void 0!==arguments[1]?arguments[1]:"utf8";var n;return O(this,void 0,void 0,(function*(){if(!this.connected||!this.publicKey)throw new Error("Wallet not connected");try{const{signature:n}=yield this._sendIframeMessage({method:"signMessage",params:{data:a.a.encode(e),display:t}});return Uint8Array.from(a.a.decode(n))}catch(i){throw new Error((null===(n=null===i||void 0===i?void 0:i.toString)||void 0===n?void 0:n.call(i))||"Failed to sign message")}}))}sign(e){let t=arguments.length>1&&void 0!==arguments[1]?arguments[1]:"utf8";return O(this,void 0,void 0,(function*(){return yield this.signMessage(e,t)}))}static isSupported(){return O(this,void 0,void 0,(function*(){return!!(yield function(){return m(this,void 0,void 0,(function*(){try{const e=window.ethereum;if(!e)return null;if(e.providers&&Array.isArray(e.providers)){const t=e.providers;for(const e of t)if(yield g(e))return e}if(e.detected&&Array.isArray(e.detected)){const t=e.detected;for(const e of t)if(yield g(e))return e}return(yield g(e))?e:null}catch(e){return console.error(e),null}}))}())}))}standardSignAndSendTransaction(){for(var e=arguments.length,t=new Array(e),n=0;n<e;n++)t[n]=arguments[n];return O(this,void 0,void 0,(function*(){if(!this.connected)throw new Error("not connected");const e=[];if(1===t.length){const{transaction:n,account:r,chain:o,options:s}=t[0],{minContextSlot:c,preflightCommitment:l,skipPreflight:h,maxRetries:d}=s||{};if(r!==this._account)throw new Error("invalid account");if(!E(o))throw new Error("invalid chain");const u=yield this.signAndSendTransaction(i.VersionedTransaction.deserialize(n),{preflightCommitment:l,minContextSlot:c,maxRetries:d,skipPreflight:h});e.push({signature:a.a.decode(u)})}else if(t.length>1)for(const n of t)e.push(...yield this.standardSignAndSendTransaction(n));return e}))}standardSignTransaction(){for(var e=arguments.length,t=new Array(e),n=0;n<e;n++)t[n]=arguments[n];return O(this,void 0,void 0,(function*(){if(!this.connected)throw new Error("not connected");const e=[];if(1===t.length){const{transaction:n,account:r,chain:o}=t[0];if(r!==this._account)throw new Error("invalid account");if(o&&!E(o))throw new Error("invalid chain");const s=yield this.signTransaction(i.VersionedTransaction.deserialize(n));e.push({signedTransaction:s.serialize()})}else if(t.length>1){let n;for(const e of t){if(e.account!==this._account)throw new Error("invalid account");if(e.chain){if(!E(e.chain))throw new Error("invalid chain");if(n){if(e.chain!==n)throw new Error("conflicting chain")}else n=e.chain}}const r=t.map((e=>{let{transaction:t}=e;return i.VersionedTransaction.deserialize(t)})),o=yield this.signAllTransactions(r);e.push(...o.map((e=>({signedTransaction:e.serialize()}))))}return e}))}standardSignMessage(){for(var e=arguments.length,t=new Array(e),n=0;n<e;n++)t[n]=arguments[n];return O(this,void 0,void 0,(function*(){if(!this.connected)throw new Error("not connected");const e=[];if(1===t.length){const{message:n,account:i}=t[0];if(i!==this._account)throw new Error("invalid account");const r=yield this.signMessage(n);e.push({signedMessage:n,signature:r})}else if(t.length>1)for(const n of t)e.push(...yield this.standardSignMessage(n));return e}))}}P.IFRAME_URL="https://widget.solflare.com/";t.default=P}}]);
//# sourceMappingURL=3.9c19cbcc.chunk.js.map