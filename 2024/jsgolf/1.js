l=[]
r=[]
o=0
require('fs').readFileSync('1','utf8').trim().split`\n`.map(x=>{[a,b]=x.split`   `;l.push(a|0);r.push(b|0)})
l.sort();r.sort()
l.map((a,i)=>o+=Math.abs(a-r[i]))
console.log(o)