/*
Pseudocódigo para calcular si un sistema empotrado es planificable o no:
1. Definir una estructura de datos para representar una tarea con los siguientes atributos:
   - tiempo de ejecución (e)
   - periodo (p)
   - worst case response time (w)
2. Crear una lista de tareas ordenadas por prioridad en nuestro caso será de menor periodo a mayor periodo.
3. Para cada tarea en la lista:
	a. Inicializar w como el tiempo de ejecución de la tarea.
	b. Repetir hasta que w no cambie:
		i. Calcular las interferencias de las tareas de mayor prioridad. (una tarea solo puede ser interferida por tareas de mayor prioridad)
		ii. Actualizar w sumando el tiempo de ejecución de la tarea y las interferencias.
	c. Si w es mayor que el periodo de la tarea, el sistema no es planificable.
4. Si todas las tareas son planificables, el sistema es planificable
 */

use std::io::stdin;

struct Tarea {
	e: u32, // tiempo de ejecución
	p: u32, // periodo
	w: u32, // worst case response time
}

fn es_planificable(tareas: &mut Vec<Tarea>) -> bool {
	// Ordenar las tareas por periodo (prioridad)
	tareas.sort_by_key(|t| t.p);

	for i in 0..tareas.len() {
		let (prior, rest) = tareas.split_at_mut(i);
		let tarea = &mut rest[0];
		tarea.w = tarea.e; // Inicializar w

		let mut iter_count = 0usize;

		loop {
			iter_count += 1;
			let w_anterior = tarea.w;
			let mut interferencia = 0u32;

			// Calcular las interferencias de las tareas de mayor prioridad
			for tarea_mayor_prioridad in prior.iter() {
				// Integer ceiling calculation: ceil(a/b) = (a + b - 1) / b
				let num_activations =
					(w_anterior + tarea_mayor_prioridad.p - 1) / tarea_mayor_prioridad.p;
				interferencia += num_activations * tarea_mayor_prioridad.e;
			}

			// Actualizar w
			tarea.w = tarea.e + interferencia;

			// Verificar si w excede el periodo
			if tarea.w > tarea.p {
				return false;
			}

			// Si w no cambia, salir del bucle e informar la iteración en la que convergió
			if tarea.w == w_anterior {
				println!("Tarea {} convergió en {} iteraciones.", i + 1, iter_count);
				break;
			}
		}
	}

	true // Todas las tareas son planificables
}

fn liu_layland(tareas: &Vec<Tarea>) -> (f32, f32) {
	let utilizacion: f32 = tareas.iter().map(|t| t.e as f32 / t.p as f32).sum();
	let n = tareas.len() as f32;
	let limite = n * (2f32.powf(1f32 / n) - 1f32);
	return (utilizacion, limite);
}

fn main() {
	/* let mut tareas = vec![
		Tarea { p: 40, e: 20, w: 0},
		Tarea { p: 100, e: 25, w: 0},
		Tarea { p: 150, e: 10, w: 0},
		Tarea { p: 500, e: 10, w: 0},
	]; */

	println!("Ingrese las tareas en el formato: <periodo> <tiempo_de_ejecución>");
	println!(
		"Una tarea por línea. Presione Ctrl+D (Linux/Mac) o Ctrl+Z (Windows) para finalizar la entrada."
	);
	let mut tareas = Vec::new();
	for (idx, line) in stdin().lines().enumerate() {
		let line = match line {
			Ok(l) if !l.trim().is_empty() => l,
			Ok(_) => continue,
			Err(err) => {
				eprintln!("Error al leer la línea {}: {}", idx + 1, err);
				continue;
			}
		};
		let mut partes = line.split_whitespace();
		let Some(p_str) = partes.next() else {
			eprintln!("Línea {} sin periodo, se ignora.", idx + 1);
			continue;
		};
		let Some(e_str) = partes.next() else {
			eprintln!("Línea {} sin tiempo de ejecución, se ignora.", idx + 1);
			continue;
		};
		match (p_str.parse::<u16>(), e_str.parse::<u16>()) {
			(Ok(p), Ok(e)) => tareas.push(Tarea {
				p: p as u32,
				e: e as u32,
				w: 0,
			}),
			_ => eprintln!("Línea {} con números inválidos, se ignora.", idx + 1),
		}
	}

	let (utilizacion, limite) = liu_layland(&tareas);
	println!(
		"Utilización: {:.4}, Límite de Liu & Layland: {:.4}",
		utilizacion, limite
	);
	if utilizacion <= limite {
		println!("Según Liu & Layland, el sistema es planificable.");
	} else {
		println!("Según Liu & Layland, no se puede asegurar que el sistema es planificable.");
	}

	if es_planificable(&mut tareas) {
		println!("El sistema es planificable.");
	} else {
		println!("El sistema no es planificable.");
	}

	println!("Tabla de tareas con sus Worst Case Response Times:");
	println!("Tarea\tPeriodo (p)\tTiempo Ejec (e)\tWCRT (w)");
	for (i, tarea) in tareas.iter().enumerate() {
		println!("T{}\t{}\t\t{}\t\t{}", i + 1, tarea.p, tarea.e, tarea.w);
	}
}
