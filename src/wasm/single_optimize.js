export function wasmSingleOptimize(optimizeConfig, artifacts, timeout = 120000) {
    const worker = new Worker(new URL("@worker/optimize_artifact.js", import.meta.url))

    return new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
            reject("Computation Timeout")
        }, timeout)

        worker.onmessage = e => {
            if (e.data.type === "ready") {
                worker.postMessage({
                    optimizeConfig,
                    artifacts,
                })
            } else {
                const results = e.data.data.results
                clearTimeout(timer)

                resolve(results)
            }
        }

        worker.onerror = () => {
            reject("Error in Calculation")
        }
    }).finally(() => {
        worker.terminate()
    })
}
