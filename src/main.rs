use rand::Rng;

fn main()
{
    let mut rng = rand::thread_rng();
    let questions = vec![ 
        "Defina Base de Datos",
        "Defina Redundancia de Datos",
        "Defina Inconsistencia de Datos",
        "Pasos del Diseño de una Base de Datos",
        "Tipos de Modelos de Datos",
        "Arquitectura ANSI-SPARC",
        "Modelos de Bases de Datos",
        "Restricciones y Ejemplos",
        "Funciones de un SGBD",
        "Componentes de un SGBD",
        "Actividades de Roles en la DB",
        "Modelo Relacional y Entidad-Relación",
        "Elementos del Modelo Relacional",
        "Características del Modelo Relacional",
        "Claves en el Modelo Relacional",
        "Problemas y Cardinalidad",
        "Generalización, Especialización y Agregación",
        "Definiciones en un BD Relacional",
        "Integridad Referencial",
        "Regla de Negocio; Definición y ejemplos",
        "Metadatos y Diccionario de Datos",
        "Normalización de Bases de Datos; Métodos de 1FN, 2FN y 3FN",
        "Definición de Usuarios y Roles en SQL",
        "Definición de Índices; Tipos de índices; Ventajas y desventajas"
    ];

    for _i in 0..2
    {
        let random_number: usize = rng.gen_range(0..questions.len());

        println!( "{}", questions[random_number] );
    }
}
