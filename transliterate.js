
    const indic = ["devanagari", "telugu", "kannada"]
    const roman = ["itrans", "iast", "slp1", "hk"]

    import init, { script_swap } from "./pkg/transliterate_ferris_wasm.js";
    const textarea1 = document.getElementById("t1");
    const textarea2 = document.getElementById("t2");
    textarea1.addEventListener("keydown", () => {
      init().then(() => {
        const t1_select = document.getElementById("t1-select").options[document.getElementById("t1-select").selectedIndex].text;
        const t2_select = document.getElementById("t2-select").options[document.getElementById("t2-select").selectedIndex].text;

        let conversion;
        if(indic.includes(t1_select) && indic.includes(t2_select)){
          conversion = 0;
        } else if(indic.includes(t1_select) && roman.includes(t2_select)){
          conversion = 1;
        } else if(roman.includes(t1_select) && roman.includes(t2_select)){
          conversion = 2;
        } else if(roman.includes(t1_select) && indic.includes(t2_select)){
          conversion = 3;
        }
        textarea2.value = script_swap(textarea1.value, t1_select, t2_select, conversion);
      });
    });

    textarea2.addEventListener("keydown", () => {
      init().then(() => {
        const t1_select = document.getElementById("t1-select").options[document.getElementById("t1-select").selectedIndex].text;
        const t2_select = document.getElementById("t2-select").options[document.getElementById("t2-select").selectedIndex].text;

        let conversion;
        if(indic.includes(t2_select) && indic.includes(t1_select)){
          conversion = 0;
        } else if(indic.includes(t2_select) && roman.includes(t1_select)){
          conversion = 1;
        } else if(roman.includes(t2_select) && roman.includes(t1_select)){
          conversion = 2;
        } else if(roman.includes(t2_select) && indic.includes(t1_select)){
          conversion = 3;
        }
        textarea1.value = script_swap(textarea2.value, t2_select, t1_select, conversion);
      });
    });
