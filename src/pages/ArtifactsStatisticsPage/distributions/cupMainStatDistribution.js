import { artifactsTagMap } from "@asset/artifacts";
import { secondaryTags } from "@asset/tags";

export default function(vm) {
    let cup = vm.$store.getters["artifacts/allArtifacts"].cup;
    if (vm.cupMainStat16Only) {
        cup = cup.filter(item => {
            return (item.level ?? 20) >= 16;
        })
    }

    let count = {};
    for (let key of artifactsTagMap["cup"]) {
        count[key] = 0;
    }
    for (let i = 0; i < cup.length; i++) {
        let name = cup[i].mainTag.name;
        count[name]++;
    }

    let options = {
        tooltip: {
            trigger: "item"
        },
        series: [
            {
                name: "Goblet Main Stat Distribution",
                type: "pie",
                radius: ["40%", "70%"],
                itemStyle: {
                    borderRadius: 10,
                    borderColor: '#fff',
                    borderWidth: 2
                },
                data: Object.keys(count).map(key => ({
                    name: secondaryTags[key].chs,
                    value: count[key]
                })).filter(item => item.value > 0),
            }
        ]
    }

    return options;
}