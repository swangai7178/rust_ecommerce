/*
 *   Copyright (c) 2025 
 *   All rights reserved.
 */

use yew::prelude::*;


#[derive(Properties, PartialEq, Clone)]
struct ProductPageProps {
    id: u32,
}

struct Product {
    id: u32,
    name: String,
    image_url: String,
    price: String,
}

#[function_component(ProductPageView)]
fn product_page(props: &ProductPageProps) -> Html {
    let products = vec![
        Product { id: 1,price:"30".to_string(), name: "Product 1".to_string(), image_url: "https://via.placeholder.com/150".to_string() },
        Product { id: 2,price:"20".to_string(), name: "Product 2".to_string(), image_url: "https://via.placeholder.com/150".to_string() },
        // Add more products as needed
    ];

    let product = products.iter().find(|p| p.id == props.id).unwrap();

    html! {
        <div class="flex flex-col items-center justify-start min-h-screen text-white">
            <header class="w-full py-4 bg-gray-800 text-white flex justify-between items-center px-8 fixed top-0 z-50">
                <h1 class="text-2xl font-bold">{"Product Details"}</h1>
            </header>
            <div class="relative w-full py-12 flex flex-col items-center mt-16">
                <img src={product.image_url.clone()} alt={product.name.clone()} class="w-full h-48 object-cover rounded-lg" />
                <h2 class="text-4xl font-bold mt-4">{ &product.name }</h2>
                <p class="text-2xl mt-2">{ format!("Price: ${}", product.price) }</p>
            </div>
        </div>
    }
}