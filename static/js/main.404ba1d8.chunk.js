(this["webpackJsonpcivic-react"]=this["webpackJsonpcivic-react"]||[]).push([[0],{108:function(e,t,n){},113:function(e,t){},137:function(e,t){},160:function(e,t,n){"use strict";n.r(t);var c=n(0),s=n.n(c),r=n(79),i=n.n(r),a=(n(108),n(6)),o=n(172),j=n(180),d=n(181),b=n(183),l=n(184),u=n(173),h=n(182),w=n(47),O=(n(138),n(2));const x=()=>{const{gatewayStatus:e,gatewayToken:t,requestGatewayToken:n}=Object(w.c)();return Object(O.jsxs)("div",{style:{border:"1px solid #ccc",margin:"1rem 0",padding:"1rem"},children:[Object(O.jsx)("h2",{children:"My Gateway Status"}),Object(O.jsxs)("p",{children:["Status: ",Object(O.jsx)("b",{children:w.b[e]})]}),t?Object(O.jsxs)("p",{children:["Found gateway token: ",Object(O.jsx)("br",{}),Object(O.jsx)("b",{children:t.publicKey.toBase58()})]}):Object(O.jsx)("p",{children:"No gateway token found yet."}),Object(O.jsx)("button",{onClick:async()=>{try{n?await n():console.warn("requestGatewayToken is undefined.")}catch(e){console.error("Error requesting pass:",e)}},children:"Request or Refresh Pass"})]})};var y=function(){var e;const t=o.a.Devnet,n=Object(a.clusterApiUrl)(t),c=[new u.a,new h.a({network:t})],s=new a.PublicKey("uniqobk8oGh4XBLMqM68K8M2zNu3CdYX7q5go7whQiv");return Object(O.jsx)(j.a,{endpoint:n,children:Object(O.jsx)(d.a,{wallets:c,autoConnect:!0,children:Object(O.jsx)(b.a,{children:Object(O.jsx)(w.a,{connection:new a.Connection(n,"processed"),cluster:"devnet",gatekeeperNetwork:s,wallet:{publicKey:c[0].publicKey,signTransaction:null===(e=c[0].signTransaction)||void 0===e?void 0:e.bind(c[0]),connected:!0},children:Object(O.jsxs)("div",{style:{margin:"2rem"},children:[Object(O.jsx)("h1",{children:"Civic Gateway Without IdentityButton"}),Object(O.jsx)(l.a,{}),Object(O.jsx)(x,{})]})})})})})};i.a.createRoot(document.getElementById("root")).render(Object(O.jsx)(s.a.StrictMode,{children:Object(O.jsx)(y,{})}))}},[[160,1,2]]]);
//# sourceMappingURL=main.404ba1d8.chunk.js.map