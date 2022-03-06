console.log('background');
chrome.tabs.onUpdated.addListener((tabId,changeInfo,tab)=>{
  console.log('listener');
});
