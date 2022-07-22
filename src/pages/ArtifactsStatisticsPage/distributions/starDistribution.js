export default function(vm) {
    let allFlat = vm.$store.getters["artifacts/allFlat"];

    let count = [0, 0, 0, 0, 0];
    for (let i = 0; i < allFlat.length; i++) {
        let star = allFlat[i].star ?? 5;
        count[star - 1]++;
    }

    let options = {
        tooltip: {
            trigger: "item"
        },
        legend: {
            // top: "0",
            left: "left",
            orient: "vertical",
        },
        series: [
            {
                name: "Quality Distribution",
                type: "pie",
                radius: ["40%", "70%"],
                label: {
                    show: false
                },
                itemStyle: {
                    borderRadius: 10,
                    borderColor: '#fff',
                    borderWidth: 2
                },
                data: [
                    { value: count[0], name: "1 Star" },
                    { value: count[1], name: "2 Star" },
                    { value: count[2], name: "3 Star" },
                    { value: count[3], name: "4 Star" },
                    { value: count[4], name: "5 Star" },
                ],
            }
        ]
    }

    return options;
}