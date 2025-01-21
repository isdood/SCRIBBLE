/*
 * harmony_mesh.chpl
 * Distributed harmony mesh for crystal wave synchronization
 *
 * Created: 2025-01-21 13:41:52 UTC
 * Author: @isdood
 */

module HarmonyMesh {
    use Time;
    use Math;
    use LinearAlgebra;
    use Random;

    // Configuration parameters
    config const meshSize = 1024;
    config const harmonicLayers = 7;
    config const resonanceThreshold = 0.001;
    config const flowDecayRate = 0.95;

    /* Harmony node state tracking */
    enum HarmonyState {
        Awakened,
        Flowing,
        Dreaming,
        Synchronized
    }

    /* Represents a point in the harmony mesh */
    record HarmonyPoint {
        var resonance: real;
        var flow: complex;
        var harmonics: [1..harmonicLayers] real;
        var state: HarmonyState;
        var lastUpdate: real;
    }

    /* Wave flow characteristics */
    record FlowPattern {
        var intensity: real;
        var direction: 2*real;
        var frequency: real;
        var phaseShift: real;
    }

    /* Main harmony mesh controller */
    class HarmonyMeshController {
        var mesh: [1..meshSize, 1..meshSize] HarmonyPoint;
        var flows: [1..meshSize, 1..meshSize] FlowPattern;
        var timeStamp: real;
        var resonancePoints: domain(2);

        /* Initialize the harmony mesh */
        proc init() {
            this.timeStamp = getCurrentTime();
            initializeMesh();
            setupResonancePoints();
        }

        /* Set up initial mesh state */
        proc initializeMesh() {
            forall (i,j) in mesh.domain {
                var baseFreq = 432.0 * (1.0 + random.getNext() * 0.1);
                mesh[i,j] = new HarmonyPoint(
                    resonance = 1.0,
                    flow = exp(2.0*pi*1.0i * random.getNext()),
                                             harmonics = generateHarmonics(baseFreq),
                                             state = HarmonyState.Dreaming,
                                             lastUpdate = timeStamp
                );
            }
        }

        /* Initialize resonance points */
        proc setupResonancePoints() {
            resonancePoints = {1..meshSize, 1..meshSize};
            var numPoints = (meshSize * meshSize) / 100; // 1% of mesh points

            for 1..numPoints {
                var i = random.getNext(1, meshSize+1);
                var j = random.getNext(1, meshSize+1);
                resonancePoints += (i,j);
                awakenNode(i, j);
            }
        }

        /* Generate harmonic frequencies */
        proc generateHarmonics(baseFreq: real): [1..harmonicLayers] real {
            var harmonics: [1..harmonicLayers] real;
            for h in 1..harmonicLayers {
                harmonics[h] = baseFreq * h * (1.0 + random.getNext() * 0.01);
            }
            return harmonics;
        }

        /* Propagate harmony through the mesh */
        proc propagateHarmony(dt: real) {
            forall (i,j) in mesh.domain {
                updateHarmonyPoint(i, j, dt);
                calculateFlowPatterns(i, j);
            }

            synchronizeMesh();
            timeStamp += dt;
        }

        /* Update individual harmony point */
        proc updateHarmonyPoint(i: int, j: int, dt: real) {
            var point = mesh[i,j];
            var flow = flows[i,j];

            // Update resonance based on neighboring points
            var newResonance = calculateResonance(i, j);

            // Update flow based on resonance change
            var newFlow = calculateFlow(point.flow, newResonance, dt);

            // Update harmonics
            var newHarmonics = updateHarmonics(point.harmonics, newResonance);

            // Determine new state
            var newState = determineHarmonyState(newResonance, point.resonance);

            // Apply updates
            point.resonance = newResonance;
            point.flow = newFlow;
            point.harmonics = newHarmonics;
            point.state = newState;
            point.lastUpdate = timeStamp;

            mesh[i,j] = point;
        }

        /* Calculate resonance from neighboring points */
        proc calculateResonance(i: int, j: int): real {
            var neighborResonance = 0.0;
            var count = 0;

            for (ni,nj) in getNeighbors(i, j) {
                neighborResonance += mesh[ni,nj].resonance;
                count += 1;
            }

            return (neighborResonance / count) * flowDecayRate;
        }

        /* Get valid neighboring points */
        iter getNeighbors(i: int, j: int) {
            for (di,dj) in [(-1,0), (1,0), (0,-1), (0,1)] {
                var ni = i + di;
                var nj = j + dj;
                if mesh.domain.contains((ni,nj)) then
                    yield (ni,nj);
            }
        }

        /* Calculate new flow based on resonance */
        proc calculateFlow(currentFlow: complex, newResonance: real, dt: real): complex {
            return currentFlow * exp(2.0*pi*1.0i * newResonance * dt);
        }

        /* Update harmonic frequencies */
        proc updateHarmonics(current: [1..harmonicLayers] real, resonance: real): [1..harmonicLayers] real {
            var updated: [1..harmonicLayers] real;
            for h in 1..harmonicLayers {
                updated[h] = current[h] * (1.0 + (resonance - 1.0) * 0.1);
            }
            return updated;
        }

        /* Calculate flow patterns */
        proc calculateFlowPatterns(i: int, j: int) {
            var pattern = flows[i,j];
            var point = mesh[i,j];

            // Calculate flow intensity
            pattern.intensity = abs(point.flow);

            // Calculate flow direction
            if i > 1 && j > 1 {
                pattern.direction = (
                    abs(mesh[i,j].resonance - mesh[i-1,j].resonance),
                                     abs(mesh[i,j].resonance - mesh[i,j-1].resonance)
                );
            }

            // Update frequency and phase
            pattern.frequency = point.harmonics[1];  // Base frequency
            pattern.phaseShift = arg(point.flow);

            flows[i,j] = pattern;
        }

        /* Determine harmony state based on resonance */
        proc determineHarmonyState(newRes: real, oldRes: real): HarmonyState {
            var change = abs(newRes - oldRes);

            if change < resonanceThreshold then
                return HarmonyState.Synchronized;
            else if newRes > 1.0 then
                return HarmonyState.Awakened;
            else if change > resonanceThreshold * 10.0 then
                return HarmonyState.Flowing;
            else
                return HarmonyState.Dreaming;
        }

        /* Awaken a node to start resonance */
        proc awakenNode(i: int, j: int) {
            var point = mesh[i,j];
            point.resonance = 2.0;  // Increased resonance
            point.state = HarmonyState.Awakened;
            mesh[i,j] = point;
        }

        /* Find synchronized regions */
        proc findSynchronizedRegions(): domain(2) {
            var syncRegion: domain(2);

            forall (i,j) in mesh.domain with (+ reduce syncRegion) {
                if mesh[i,j].state == HarmonyState.Synchronized {
                    syncRegion += (i,j);
                }
            }

            return syncRegion;
        }

        /* Get mesh statistics */
        proc getMeshStatistics() {
            var awakenedCount = 0;
            var flowingCount = 0;
            var dreamingCount = 0;
            var synchronizedCount = 0;

            forall point in mesh with (+ reduce awakenedCount,
                + reduce flowingCount,
                + reduce dreamingCount,
                + reduce synchronizedCount) {
                select point.state {
                    when HarmonyState.Awakened do awakenedCount += 1;
                    when HarmonyState.Flowing do flowingCount += 1;
                    when HarmonyState.Dreaming do dreamingCount += 1;
                    when HarmonyState.Synchronized do synchronizedCount += 1;
                }
                }

                return (awakenedCount, flowingCount, dreamingCount, synchronizedCount);
        }
    }
}
