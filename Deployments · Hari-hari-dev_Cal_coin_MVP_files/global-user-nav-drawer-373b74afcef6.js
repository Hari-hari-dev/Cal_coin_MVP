"use strict";(globalThis.webpackChunk=globalThis.webpackChunk||[]).push([["global-user-nav-drawer"],{85663:(e,t,r)=>{var a=r(72245),s=r(74848),n=r(38621),i=r(80497),o=r(47139),c=r(30595),l=r(30631),d=r(30729),u=r(96540),h=r(92240),m=r(22084),p=r(92979),y=r(75014),f=r(91246),x=r(22868),j=r(5225),g=r(71312),_=r(28784);async function A(e){return e.set("_method","put"),(await (0,_.DI)("/users/status",{method:"POST",headers:{"X-Requested-With":"XMLHttpRequest",Accept:"application/json"},body:e})).json()}function w({onClose:e}){let[t,r]=(0,u.useState)(!1),a=(0,u.useRef)(null),i=(0,u.useId)(),o=(0,u.useCallback)(t=>{t.preventDefault(),e(A(new FormData(t.target)))},[e]),l=(0,u.useCallback)(()=>{A(new FormData),e(Promise.resolve({}))},[e]);return(0,u.useEffect)(()=>{let e=a.current;if(e)return e.addEventListener("load",t),()=>{e.removeEventListener("load",t)};function t(){r(!0)}},[a]),(0,s.jsx)(f.l,{width:"large",title:"Edit status",onClose:e,footerButtons:[{buttonType:"normal",content:"Clear status",onClick:l},{buttonType:"primary",type:"submit",content:"Set status",form:i,disabled:!t}],children:(0,s.jsx)("form",{id:i,onSubmit:o,className:"user-status-dialog-fragment js-user-status-container",children:(0,s.jsxs)("include-fragment",{src:"/users/status",accept:"text/fragment+html",ref:a,...(0,h.G)("user-status-dialog-include-fragment"),children:[(0,s.jsx)("p",{className:"text-center mt-3","data-hide-on-error":!0,children:(0,s.jsx)(c.A,{})}),(0,s.jsxs)("p",{className:"flash flash-error mb-0 mt-2","data-show-on-error":!0,hidden:!0,children:[(0,s.jsx)(n.AlertIcon,{}),"Sorry, something went wrong and we were not able to fetch the user settings form"]})]})})})}try{w.displayName||(w.displayName="UserStatusDialog")}catch{}let v={emojiIcon:"styles-module__emojiIcon--AXyd_",emojiContainer:"styles-module__emojiContainer--yBvrp"};var S=r(20053);function b(e){let{class:t}=e.attributes,r=(0,S.$)(t,v.emojiIcon),a={...e.attributes};switch(delete a.class,e.tag){case"g-emoji":return(0,u.createElement)(e.tag,{...a,class:r},e.raw);case"img":return(0,s.jsx)("img",{alt:"",src:e.imgPath,...a,className:r})}return(0,s.jsx)(s.Fragment,{})}try{b.displayName||(b.displayName="Emoji")}catch{}var N=r(24664);function I({title:e,error:t,onClose:r}){return(0,s.jsx)(f.l,{onClose:r,title:e,children:(0,s.jsx)(f.l.Body,{children:(0,s.jsxs)(N.A,{children:[(0,s.jsx)(N.A.Visual,{children:(0,s.jsx)(n.AlertIcon,{size:"medium",className:"fgColor-danger"})}),t]})})})}try{I.displayName||(I.displayName="ErrorDialog")}catch{}var C=r(89323),P=r(87330),E=r(47767),L=r(97156);function T({addAccountPath:e,canAddAccount:t,switchAccountPath:r,stashedAccounts:a,loginAccountPath:i,setError:o}){let[c,l]=(0,u.useState)(!1),d=null===a,h=!d&&a.length>0;return(0,s.jsxs)(C.W,{open:c,onOpenChange:l,children:[(0,s.jsx)(C.W.Anchor,{children:(0,s.jsx)(P.K,{icon:n.ArrowSwitchIcon,"aria-label":"Account switcher",variant:"invisible",tooltipDirection:"s"})}),d?(0,s.jsx)(k,{}):h?(0,s.jsx)(O,{stashedAccounts:a,canAddAccount:t,addAccountPath:e,switchAccountPath:r,loginAccountPath:i,setError:o}):(0,s.jsx)(R,{addAccountPath:e})]})}function k(){return(0,s.jsx)(C.W.Overlay,{align:"end",children:(0,s.jsx)(i.l,{children:(0,s.jsxs)(i.l.Item,{children:[(0,s.jsx)(i.l.LeadingVisual,{children:(0,s.jsx)(c.A,{size:"small"})}),"Loading..."]})})})}function R({addAccountPath:e}){return(0,s.jsx)(C.W.Overlay,{align:"end",children:(0,s.jsx)(i.l,{children:(0,s.jsx)(U,{href:e})})})}function U({href:e,inactive:t}){return(0,s.jsxs)(i.l.LinkItem,{href:e||void 0,inactiveText:t||void 0,children:[(0,s.jsx)(i.l.LeadingVisual,{children:(0,s.jsx)(n.PersonAddIcon,{})}),"Add account"]})}function O({addAccountPath:e,canAddAccount:t,switchAccountPath:r,stashedAccounts:a,loginAccountPath:o,setError:c}){return(0,s.jsx)(C.W.Overlay,{align:"end",width:"small",children:(0,s.jsxs)(i.l,{children:[(0,s.jsxs)(i.l.Group,{children:[(0,s.jsx)(i.l.GroupHeading,{children:"Switch account"}),a.map(e=>"number"==typeof e.userSessionId?(0,s.jsx)(D,{switchAccountPath:r,account:e,setError:c},e.login):(0,s.jsx)(G,{loginAccountPath:o,account:e},e.login)),(0,s.jsx)(i.l.Divider,{})]}),(0,s.jsx)(U,{href:t?e:void 0,inactive:t?void 0:"Maximum accounts reached"}),(0,s.jsxs)(i.l.LinkItem,{href:"/logout",children:[(0,s.jsx)(i.l.LeadingVisual,{children:(0,s.jsx)(n.SignOutIcon,{})}),"Sign out..."]})]})})}function D({account:e,switchAccountPath:t,setError:r}){return(0,s.jsxs)(i.l.Item,{onSelect:()=>H(t,e.userSessionId,r),children:[(0,s.jsx)(i.l.LeadingVisual,{children:(0,s.jsx)(x.r,{src:e.avatarUrl,size:20})}),e.login,(0,s.jsx)(i.l.Description,{truncate:!0,children:e.name})]})}function G({account:e,loginAccountPath:t}){let r=(0,E.zy)(),a=(0,u.useMemo)(()=>{let r=new URL(t,L.fV.toString());return r.searchParams.set("login",e.login),r.searchParams.set("return_to",L.fV.toString()),r.toString()},[t,e.login,r.key]);return(0,s.jsxs)(i.l.LinkItem,{href:a,children:[(0,s.jsx)(i.l.LeadingVisual,{children:(0,s.jsx)(x.r,{src:e.avatarUrl,size:20,className:"inactive-user-avatar"})}),e.login,(0,s.jsx)(i.l.Description,{truncate:!0,children:e.name})]})}async function H(e,t,r){try{let a=new FormData;a.append("user_session_id",String(t)),a.append("from","nav_panel");let s=await (0,_.DI)(e,{method:"POST",body:a,headers:{Accept:"application/json"}});if(s.ok){let e=await s.json();e.success?window.location.reload():"emu_sso_redirect"===e.reason&&e.redirect_to?window.location.href=e.redirect_to:r(M("An error has occurred while switching accounts. Please try again."))}else{let{error:e,reason:t}=await s.json();r(M(e,t))}}catch{r(M("An error occurred while switching accounts. Please try again."))}}function M(e,t){return{title:"Switch account",error:"enterprise access denied"===t?(0,s.jsx)(N.A.Description,{children:e}):(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(N.A.Heading,{children:"Unable to switch to the selected account."}),(0,s.jsx)(N.A.Description,{children:"Please try again. If the issue persists, try adding the account again."})]})}}try{T.displayName||(T.displayName="AccountSwitcher")}catch{}try{k.displayName||(k.displayName="AccountSwitcherOverlayLoading")}catch{}try{R.displayName||(R.displayName="AccountSwitcherOverlayEmpty")}catch{}try{U.displayName||(U.displayName="AddAccountLinkItem")}catch{}try{O.displayName||(O.displayName="AccountSwitcherOverlayHasAccounts")}catch{}try{D.displayName||(D.displayName="StashedAccountItem")}catch{}try{G.displayName||(G.displayName="InactiveStashedAccountItem")}catch{}var F=r(22629);let z=(0,j.A)(async function(e){let t=await fetch(e);if(!t.ok)throw Error(`Failed to fetch data from ${e}`);return t.json()}),Y={fetchError:!0,userStatus:{},hasUnseenFeatures:!1,stashedAccounts:[]};function V({href:e,icon:t,analyticsCategory:r="Global navigation",analyticsAction:a,analyticsLabel:n,children:o}){let{sendClickAnalyticsEvent:c}=(0,g.S)(),l=(0,u.useCallback)(()=>{c({category:r,action:a,label:n})},[c,r,a,n]);return(0,s.jsxs)(i.l.LinkItem,{href:e,onClick:l,children:[(0,s.jsx)(i.l.LeadingVisual,{children:(0,s.jsx)(d.A,{icon:t})}),o]})}let W=(0,u.memo)(function({lazyLoadItemData:e,onClick:t}){return(0,s.jsxs)(i.l.Item,{...(0,h.G)("global-user-nav-set-status-item"),onSelect:t,children:[(0,s.jsx)(i.l.LeadingVisual,{children:e?.userStatus?.emojiAttributes?(0,s.jsx)(b,{...e?.userStatus.emojiAttributes}):(0,s.jsx)(d.A,{icon:n.SmileyIcon})}),e?(0,s.jsx)(m.oG,{className:v.emojiContainer,unverifiedHTML:e.userStatus.messageHtml||"Set status"}):(0,s.jsx)(p.O,{height:"md"})]})});function B(e){let t=e.lazyLoadItemData?.enterpriseTrialUrl;return t?(0,s.jsxs)(V,{href:t,icon:n.UploadIcon,analyticsCategory:"start_a_free_trial",analyticsAction:"click_to_set_up_enterprise_trial",analyticsLabel:"ref_loc:side_panel;ref_cta:try_enterprise",children:["Try Enterprise",(0,s.jsx)(i.l.TrailingVisual,{children:(0,s.jsx)(o.A,{variant:"primary",children:"Free"})})]}):(0,s.jsx)(V,{href:"/account/choose?action=upgrade",icon:n.UploadIcon,analyticsAction:"UPGRADE_PLAN",children:"Upgrade"})}function X({onClose:e,login:t}){return(0,s.jsx)(f.l,{title:"Feature preview dialog",sx:{width:960},onClose:e,renderBody:()=>(0,s.jsx)(f.l.Body,{className:"p-0",children:(0,s.jsxs)("include-fragment",{src:`/users/${t}/feature_previews`,children:[(0,s.jsx)("p",{className:"text-center mt-3","data-hide-on-error":!0,children:(0,s.jsx)(c.A,{})}),(0,s.jsxs)("p",{className:"flash flash-error mb-0 mt-2","data-show-on-error":!0,hidden:!0,children:[(0,s.jsx)(n.AlertIcon,{}),"Sorry, something went wrong and we were not able to fetch the feature previews"]})]})})})}function q(e){let[t,r]=(0,u.useState)(null),[a,c]=(0,u.useState)(!1),[h,m]=(0,u.useState)(!1),{onClose:p,owner:y}=e,j=`/${y.login}`,{sendClickAnalyticsEvent:_}=(0,g.S)(),A=(0,u.useCallback)(()=>{c(!0),_({category:"Global navigation",action:"USER_STATUS"})},[_]),v=(0,u.useCallback)(async e=>{if(c(!1),e&&"string"!=typeof e&&t)try{let a=await e;r({...t,userStatus:a})}catch{}},[t]),S=(0,u.useCallback)(()=>{m(!0),_({category:"Global navigation",action:"FEATURE_PREVIEW"})},[_]);(0,u.useEffect)(()=>{t||(async()=>{try{let t=await z(e.lazyLoadItemDataFetchUrl);r({fetchError:!1,...t})}catch{r(Y)}})()},[e.lazyLoadItemDataFetchUrl,t]);let b=(0,u.useCallback)(({dialogLabelId:r})=>(0,s.jsxs)("div",{className:"d-flex pr-3 pl-3 pt-3",id:r,"aria-label":"User navigation",role:"heading","aria-level":1,children:[(0,s.jsx)("div",{className:"d-flex flex-1",children:(0,s.jsxs)("div",{className:"d-flex",children:[(0,s.jsx)(x.r,{src:y.avatarUrl,size:32}),(0,s.jsxs)("div",{className:"lh-condensed overflow-hidden d-flex flex-column flex-justify-center ml-2 f5 mr-auto",children:[(0,s.jsx)("div",{className:"text-bold",children:(0,s.jsx)(l.A,{title:y.login,maxWidth:175,children:y.login})}),(0,s.jsx)("div",{className:"fgColor-muted",children:(0,s.jsx)(l.A,{title:y.name,maxWidth:175,children:y.name})})]})]})}),e.showAccountSwitcher&&(0,s.jsx)(T,{canAddAccount:e.canAddAccount,addAccountPath:e.addAccountPath,switchAccountPath:e.switchAccountPath,stashedAccounts:t?.stashedAccounts??null,loginAccountPath:e.loginAccountPath,setError:C}),(0,s.jsx)(f.l.CloseButton,{onClose:()=>p("close-button")})]}),[p,y,t?.stashedAccounts,e.canAddAccount,e.addAccountPath,e.switchAccountPath,e.loginAccountPath,e.showAccountSwitcher]),[N,C]=(0,u.useState)(!1);return N?(0,s.jsx)(I,{...N,onClose:()=>C(!1)}):(0,s.jsxs)(f.l,{onClose:e.onClose,width:"medium",position:"right",renderHeader:b,children:[a&&(0,s.jsx)(w,{onClose:v}),h&&(0,s.jsx)(X,{onClose:()=>m(!1),login:e.owner.login}),(0,s.jsxs)(i.l,{variant:"full",children:[(0,s.jsx)(W,{lazyLoadItemData:t,onClick:A}),(0,s.jsx)(i.l.Divider,{}),(0,s.jsx)(V,{href:j,icon:n.PersonIcon,analyticsAction:"PROFILE",children:"Your profile"}),(0,s.jsx)(V,{href:`${j}?tab=repositories`,icon:n.RepoIcon,analyticsAction:"YOUR_REPOSITORIES",children:"Your repositories"}),e.showCopilot&&(0,s.jsx)(V,{href:"/settings/copilot",icon:n.CopilotIcon,analyticsCategory:"try_copilot",analyticsAction:"click_to_try_copilot",analyticsLabel:"ref_loc:side_panel;ref_cta:your_copilot",children:"Your Copilot"}),(0,s.jsx)(V,{href:e.projectsPath,icon:n.ProjectIcon,analyticsAction:"YOUR_PROJECTS",children:"Your projects"}),(0,s.jsx)(V,{href:`${j}?tab=stars`,icon:n.StarIcon,analyticsAction:"YOUR_STARS",children:"Your stars"}),e.showGists&&(0,s.jsx)(V,{href:e.gistsUrl,icon:n.CodeSquareIcon,analyticsAction:"YOUR_GISTS",children:"Your gists"}),e.showOrganizations&&(0,s.jsx)(V,{href:"/settings/organizations",icon:n.OrganizationIcon,analyticsAction:"YOUR_ORGANIZATIONS",children:"Your organizations"}),e.showEnterprises&&(0,s.jsx)(V,{href:"/settings/enterprises",icon:n.GlobeIcon,analyticsCategory:"enterprises_more_discoverable",analyticsAction:"click_your_enterprises",analyticsLabel:"ref_loc:side_panel;ref_cta:your_enterprises;is_navigation_redesign:true",children:"Your enterprises"}),e.showEnterprise&&(0,s.jsx)(V,{href:e.yourEnterpriseUrl,icon:n.GlobeIcon,analyticsAction:"YOUR_ENTERPRISE",children:"Your enterprise"}),e.showSponsors&&(0,s.jsx)(V,{href:"/sponsors/accounts",icon:n.HeartIcon,analyticsAction:"SPONSORS",children:"Your sponsors"}),(0,s.jsx)(i.l.Divider,{}),(0,s.jsx)(F.O,{...e.createMenuProps}),e.showUpgrade&&(0,s.jsx)(B,{lazyLoadItemData:t}),e.showFeaturesPreviews&&(0,s.jsxs)(i.l.Item,{onSelect:S,children:[(0,s.jsx)(i.l.LeadingVisual,{children:(0,s.jsx)(d.A,{icon:n.BeakerIcon})}),t?.hasUnseenFeatures&&(0,s.jsx)(i.l.TrailingVisual,{children:(0,s.jsx)(o.A,{variant:"accent",children:"New"})}),(0,s.jsx)("span",{children:"Feature preview"})]}),(0,s.jsx)(V,{href:"/settings/profile",icon:n.GearIcon,analyticsAction:"SETTINGS",children:"Settings"}),e.showEnterpriseSettings&&(0,s.jsx)(V,{href:e.enterpriseSettingsUrl,icon:n.GlobeIcon,analyticsAction:"ENTERPRISE_SETTINGS",children:"Enterprise settings"}),(0,s.jsx)(i.l.Divider,{}),(0,s.jsx)(V,{href:"https://github.com/home",icon:n.BrowserIcon,analyticsAction:"MARKETINGWEBSITE",children:"GitHub Website"}),(0,s.jsx)(V,{href:e.docsUrl,icon:n.BookIcon,analyticsAction:"DOCS",children:"GitHub Docs"}),(0,s.jsx)(V,{href:e.supportUrl,icon:n.PeopleIcon,analyticsAction:"SUPPORT",children:"GitHub Support"}),(0,s.jsx)(V,{href:"https://community.github.com",icon:n.CommentDiscussionIcon,analyticsAction:"COMMUNITY",children:"GitHub Community"}),(0,s.jsx)(i.l.Divider,{}),(0,s.jsx)(V,{href:"/logout",icon:n.SignOutIcon,analyticsAction:"LOGOUT",children:"Sign out"})]})]})}function $(e){let{open:t,setOpen:r,ref:a}=(0,y.Mm)(e.reactPartialAnchor),n=(0,u.useCallback)(()=>{r(!1),setTimeout(()=>{a.current?.focus()})},[r,a]);return t?(0,s.jsx)(q,{...e,onClose:n}):(0,s.jsx)(s.Fragment,{})}function J(e){return e.reactPartialAnchor?(0,s.jsx)($,{...e,reactPartialAnchor:e.reactPartialAnchor}):(0,s.jsx)(q,{...e})}try{V.displayName||(V.displayName="NavLink")}catch{}try{W.displayName||(W.displayName="UserStatusNavItem")}catch{}try{B.displayName||(B.displayName="UpgradeNavItem")}catch{}try{X.displayName||(X.displayName="FeaturePreviewDialog")}catch{}try{q.displayName||(q.displayName="GlobalUserNavDrawerDialog")}catch{}try{$.displayName||($.displayName="ExternallyAnchoredGlobalUserNavDrawer")}catch{}try{J.displayName||(J.displayName="GlobalUserNavDrawer")}catch{}(0,a.k)("global-user-nav-drawer",{Component:J})},92240:(e,t,r)=>{r.d(t,{G:()=>a});let a=e=>({"data-testid":e})},28784:(e,t,r)=>{function a(e,t={}){!function(e){if(new URL(e,window.location.origin).origin!==window.location.origin)throw Error("Can not make cross-origin requests from verifiedFetch")}(e);let r={...t.headers,"GitHub-Verified-Fetch":"true","X-Requested-With":"XMLHttpRequest"};return fetch(e,{...t,headers:r})}function s(e,t){let r={...t?.headers??{},Accept:"application/json","Content-Type":"application/json"},s=t?.body?JSON.stringify(t.body):void 0;return a(e,{...t,body:s,headers:r})}function n(e,t={}){let r={...t.headers,"GitHub-Is-React":"true"};return a(e,{...t,headers:r})}function i(e,t){let r={...t?.headers??{},"GitHub-Is-React":"true"};return s(e,{...t,headers:r})}r.d(t,{DI:()=>a,QJ:()=>n,Sr:()=>i,lS:()=>s})},22868:(e,t,r)=>{r.d(t,{r:()=>o});var a=r(74848),s=r(97156),n=r(47258),i=r(96540);let o=(0,i.forwardRef)(function({src:e,size:t=20,...r},o){let c=(0,i.useMemo)(()=>{let r=new URL(e,s.fV.origin);return r.searchParams.has("size")||r.searchParams.has("s")||r.searchParams.set("size",String(2*Number(t))),r.toString()},[e,t]);return(0,a.jsx)(n.A,{ref:o,src:c,size:t,"data-testid":"github-avatar",...r})});try{o.displayName||(o.displayName="GitHubAvatar")}catch{}},22084:(e,t,r)=>{r.d(t,{mo:()=>p,oG:()=>u,yx:()=>h});var a,s=r(74848),n=r(75177),i=r(52464),o=r(42838),c=r.n(o),l=r(96540);function d(e){if("html"in e&&void 0!==e.html){let{html:t,...r}=e;return{safeHTML:t,props:r}}let{unverifiedHTML:t,unverifiedHTMLConfig:r,...a}=e,s={...r,RETURN_DOM:!1,RETURN_DOM_FRAGMENT:!1};return{safeHTML:c().sanitize(t,s),props:a}}let u=m(n.A);u.displayName="SafeHTMLBox";let h=m(i.A);function m(e){return(0,l.forwardRef)((t,r)=>{let{safeHTML:a,props:n}=d(t);return(0,s.jsx)(e,{ref:r,...n,dangerouslySetInnerHTML:a?{__html:a}:void 0})})}h.displayName="SafeHTMLText";let p=(0,l.forwardRef)((e,t)=>{let{safeHTML:r,props:a}=d(e);return(0,s.jsx)("div",{ref:t,...a,dangerouslySetInnerHTML:r?{__html:r}:void 0})});p.displayName="SafeHTMLDiv";try{(a=SafeHTMLComponent).displayName||(a.displayName="SafeHTMLComponent")}catch{}},92979:(e,t,r)=>{r.d(t,{O:()=>u});var a=r(75177),s=r(59299),n=r(44999),i=r(38267);let o=(0,i.i7)(["0%{transform:translateX(-100%);}50%{transform:translateX(100%);}100%{transform:translateX(100%);}"]),c=(0,i.AH)(["animation:"," 1.5s infinite linear;"],o),l=(0,i.i7)(["0%{opacity:.3;}10%{opacity:1;}100%{opacity:.3;}"]),d=(0,i.AH)(["animation:"," 2s infinite linear;"],l),u=(0,i.Ay)(a.A).withConfig({displayName:"LoadingSkeleton",componentId:"sc-bcbf24f9-0"})(["position:relative;overflow:hidden;mask-image:radial-gradient(white,black);",";&::after{",";background:linear-gradient(90deg,transparent,",",transparent);content:'';position:absolute;transform:translateX(-100%);bottom:0;left:0;right:0;top:0;}background-color:",";border-radius:",";display:block;height:1.2em;"," width:",";height:",";",""],({animationStyle:e})=>"pulse"===e&&d,({animationStyle:e})=>"pulse"!==e&&c,(0,s.Jt)("colors.neutral.subtle"),(0,s.Jt)("colors.neutral.subtle"),({theme:e,variant:t})=>{switch(t){case"rounded":return(0,s.Jt)("radii.1")(e);case"pill":return"100px";case"elliptical":return"50%";default:return"0"}},({variant:e})=>"elliptical"===e&&{borderRadius:"50%"},({width:e})=>{switch(e){case"random":return`${Math.floor(40*Math.random()+40)}%`;case"xl":return"32px";case"lg":return"24px";case"md":return"20px";case"sm":return"16px";default:return e}},({height:e})=>{switch(e){case"xl":return"32px";case"lg":return"24px";case"md":return"20px";case"sm":return"16px";default:return e}},n.A);try{u.displayName||(u.displayName="LoadingSkeleton")}catch{}}},e=>{var t=t=>e(e.s=t);e.O(0,["primer-react","react-core","react-lib","octicons-react","vendors-node_modules_primer_behaviors_dist_esm_index_mjs","vendors-node_modules_tanstack_query-core_build_modern_queryClient_js","vendors-node_modules_emotion_is-prop-valid_dist_emotion-is-prop-valid_esm_js-node_modules_emo-37e3d5","vendors-node_modules_github_mini-throttle_dist_index_js-node_modules_stacktrace-parser_dist_s-e7dcdd","vendors-node_modules_oddbird_popover-polyfill_dist_popover-fn_js","vendors-node_modules_dompurify_dist_purify_js","ui_packages_failbot_failbot_ts","ui_packages_promise-with-resolvers-polyfill_promise-with-resolvers-polyfill_ts-ui_packages_re-8d43b0"],()=>t(85663)),e.O()}]);
//# sourceMappingURL=global-user-nav-drawer-52708af11fea.js.map