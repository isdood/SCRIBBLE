/*
 * wave_lattice.chpl
 * Distributed crystal wave lattice for harmonic pattern formation
 *
 * Created: 2025-01-21 13:42:53 UTC
 * Author: @isdood
 */

module WaveLattice {
    use Time;
    use Math;
    use LinearAlgebra;
    use Random;

    // Lattice configuration
    config const latticeSize = 1024;
    config const harmonicDepth = 9;  // Crystal harmonic layers
    config const stabilityThreshold = 0.0001;
    config const maxResonance = 3.0;

    /* Lattice node vibrational states */
    enum LatticeState {
        Vibrating,
        Resonant,
        Dormant,
        Transitioning
    }

    /* Crystal lattice node */
    record LatticeNode {
        var energy: complex;
        var harmonics: [1..harmonicDepth] real;
        var amplitude: real;
        var phase: real;
        var state: LatticeState;
        var connections: [1..4] bool;  // North, East, South, West
    }

    /* Wave transmission characteristics */
    record WaveTransmission {
        var power: real;
        var direction: 2*real;
        var coherence: real;
        var stability: real;
    }

    /* Primary lattice controller */
    class WaveLatticeController {
        var lattice: [1..latticeSize, 1..latticeSize] LatticeNode;
        var transmissions: [1..latticeSize, 1..latticeSize] WaveTransmission;
        var timeStep: real = 0.0;
        var activeSites: domain(2);

        /* Initialize the wave lattice */
        proc init() {
            timeStep = getCurrentTime();
            initializeLattice();
            setupConnections();
        }

        /* Set up initial lattice configuration */
        proc initializeLattice() {
            forall (i,j) in lattice.domain {
                var baseFrequency = 432.0 * (1.0 + random.getNext() * 0.01);
                lattice[i,j] = new LatticeNode(
                    energy = exp(2.0*pi*1.0i * random.getNext()),
                                               harmonics = generateHarmonicSeries(baseFrequency),
                                               amplitude = 1.0,
                                               phase = 0.0,
                                               state = LatticeState.Dormant,
                                               connections = [true, true, true, true]
                );
            }
        }

        /* Generate harmonic frequency series */
        proc generateHarmonicSeries(base: real): [1..harmonicDepth] real {
            var series: [1..harmonicDepth] real;
            for h in 1..harmonicDepth {
                // Golden ratio influence on harmonics
                var goldenFactor = (1.0 + sqrt(5.0)) / 2.0;
                series[h] = base * h * (1.0 + (h-1) * (goldenFactor - 1.0) * 0.01);
            }
            return series;
        }

        /* Initialize lattice connections */
        proc setupConnections() {
            forall (i,j) in lattice.domain {
                updateNodeConnections(i, j);
            }
        }

        /* Update node connections based on position */
        proc updateNodeConnections(i: int, j: int) {
            var node = lattice[i,j];
            node.connections[1] = i > 1;          // North
            node.connections[2] = j < latticeSize; // East
            node.connections[3] = i < latticeSize; // South
            node.connections[4] = j > 1;          // West
            lattice[i,j] = node;
        }

        /* Propagate waves through the lattice */
        proc propagateWaves(dt: real) {
            timeStep += dt;

            forall (i,j) in lattice.domain {
                updateNode(i, j, dt);
                calculateTransmission(i, j);
            }

            synchronizeLattice();
        }

        /* Update individual node state */
        proc updateNode(i: int, j: int, dt: real) {
            var node = lattice[i,j];
            var neighborEnergy = gatherNeighborEnergy(i, j);

            // Update energy state
            var newEnergy = calculateNewEnergy(node, neighborEnergy, dt);

            // Update harmonics
            var newHarmonics = updateHarmonics(node.harmonics, newEnergy);

            // Calculate new amplitude and phase
            var newAmplitude = abs(newEnergy);
            var newPhase = arg(newEnergy);

            // Determine new state
            var newState = determineLatticeState(node, newEnergy);

            // Apply updates
            node.energy = newEnergy;
            node.harmonics = newHarmonics;
            node.amplitude = newAmplitude;
            node.phase = newPhase;
            node.state = newState;

            lattice[i,j] = node;
        }

        /* Gather energy from neighboring nodes */
        proc gatherNeighborEnergy(i: int, j: int): complex {
            var totalEnergy: complex = 0.0 + 0.0i;
            var count = 0;

            // Check all connected neighbors
            for (ni, nj) in getConnectedNeighbors(i, j) {
                totalEnergy += lattice[ni,nj].energy;
                count += 1;
            }

            return if count > 0 then totalEnergy / count else 0.0 + 0.0i;
        }

        /* Get connected neighboring nodes */
        iter getConnectedNeighbors(i: int, j: int) {
            const dirs = [(-1,0), (0,1), (1,0), (0,-1)];  // N, E, S, W
            for idx in 1..4 {
                if lattice[i,j].connections[idx] {
                    var ni = i + dirs[idx][1];
                    var nj = j + dirs[idx][2];
                    if lattice.domain.contains((ni,nj)) then
                        yield (ni,nj);
                }
            }
        }

        /* Calculate new energy state */
        proc calculateNewEnergy(node: LatticeNode, neighborEnergy: complex, dt: real): complex {
            var currentEnergy = node.energy;

            // Apply wave equation with harmonic influence
            var harmonicFactor = calculateHarmonicInfluence(node.harmonics);

            return (currentEnergy + neighborEnergy) * exp(2.0*pi*1.0i * harmonicFactor * dt);
        }

        /* Update harmonic frequencies */
        proc updateHarmonics(current: [1..harmonicDepth] real, newEnergy: complex): [1..harmonicDepth] real {
            var energyMagnitude = abs(newEnergy);
            var updated: [1..harmonicDepth] real;

            for h in 1..harmonicDepth {
                var adjustment = (energyMagnitude - 1.0) * 0.1;
                updated[h] = current[h] * (1.0 + adjustment);
            }

            return updated;
        }

        /* Calculate harmonic influence */
        proc calculateHarmonicInfluence(harmonics: [1..harmonicDepth] real): real {
            var influence = 0.0;
            for h in 1..harmonicDepth {
                influence += harmonics[h] / (h * h);  // Diminishing influence
            }
            return influence;
        }

        /* Determine lattice state */
        proc determineLatticeState(node: LatticeNode, newEnergy: complex): LatticeState {
            var energyChange = abs(newEnergy - node.energy);

            if energyChange < stabilityThreshold then
                return LatticeState.Resonant;
            else if abs(newEnergy) > maxResonance then
                return LatticeState.Vibrating;
            else if energyChange > stabilityThreshold * 10.0 then
                return LatticeState.Transitioning;
            else
                return LatticeState.Dormant;
        }

        /* Calculate wave transmission properties */
        proc calculateTransmission(i: int, j: int) {
            var node = lattice[i,j];
            var trans = new WaveTransmission();

            // Calculate power from energy
            trans.power = abs(node.energy);

            // Calculate direction based on energy gradients
            trans.direction = calculateEnergyGradient(i, j);

            // Calculate coherence with neighbors
            trans.coherence = calculateCoherence(i, j);

            // Calculate stability
            trans.stability = calculateStability(node);

            transmissions[i,j] = trans;
        }

        /* Calculate energy gradient */
        proc calculateEnergyGradient(i: int, j: int): 2*real {
            var gradX, gradY: real;

            if i > 1 && i < latticeSize {
                gradX = abs(lattice[i+1,j].energy) - abs(lattice[i-1,j].energy);
            }

            if j > 1 && j < latticeSize {
                gradY = abs(lattice[i,j+1].energy) - abs(lattice[i,j-1].energy);
            }

            return (gradX, gradY);
        }

        /* Calculate wave coherence */
        proc calculateCoherence(i: int, j: int): real {
            var phaseSum: complex = 0.0 + 0.0i;
            var count = 0;

            for (ni, nj) in getConnectedNeighbors(i, j) {
                phaseSum += exp(1.0i * lattice[ni,nj].phase);
                count += 1;
            }

            return if count > 0 then abs(phaseSum) / count else 0.0;
        }

        /* Calculate node stability */
        proc calculateStability(node: LatticeNode): real {
            var harmonicStability = 1.0;
            for h in 2..harmonicDepth {
                harmonicStability *= (1.0 - abs(node.harmonics[h] / node.harmonics[1] - h));
            }
            return harmonicStability;
        }
    }
}
