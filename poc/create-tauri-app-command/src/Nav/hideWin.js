import findNavWinByLabel from "./findNavWinByLabel";

export default function hideWin(setHideWin, showIn2Seconds) {
    setHideWin(prev => {
        return ++prev;
    });
    let navWin = findNavWinByLabel();
    navWin.hide();
    if(showIn2Seconds){
        setTimeout(() => {
            //show nav 
            navWin.show();
        }, 2000);
    }

}