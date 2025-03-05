/*
 * Copyright (c) 2025 
 * All rights reserved.
 */

 use std::rc::Rc;
 use std::cell::RefCell;
 use yew_router::prelude::*;
 use yew::prelude::*;
 
 mod productview;

 
 #[derive(Clone, PartialEq)]
 struct Cart {
     items: Rc<RefCell<Vec<String>>>,
 }
 
 impl Cart {
     fn new() -> Self {
         Self {
             items: Rc::new(RefCell::new(Vec::new())),
         }
     }
 
     fn add_item(&self, item: String) {
         self.items.borrow_mut().push(item);
     }
 
     fn get_items(&self) -> Vec<String> {
         self.items.borrow().clone()
     }
 }
 
 #[derive(Clone, PartialEq)]
 struct Product {
     id: u32,
     name: String,
     image_url: String,
     price: String,
 }
 
 #[derive(Routable, PartialEq, Clone, Debug)]
 enum Route {
     #[at("/")]
     Home,
     #[at("/product/:id")]
     ProductPage { id: u32 },
     #[at("/productview")]
     ProductViewPage,
 }
 
 #[function_component(Home)]
 fn home() -> Html {
     let cart = use_state(|| Cart::new());
     let link = use_navigator().unwrap();
 
     let add_to_cart = {
         let cart = cart.clone();
         Callback::from(move |item: String| {
             (*cart).add_item(item);
         })
     };
 
     let products = use_state(|| vec![
         Product { id: 1, price: "30".to_string(), name: "Product 1".to_string(), image_url: "https://img.freepik.com/free-vector/white-product-podium-with-green-tropical-palm-leaves-golden-round-arch-green-wall_87521-3023.jpg".to_string() },
         Product { id: 2, price: "50".to_string(), name: "Product 2".to_string(), image_url: "https://img.freepik.com/free-vector/white-product-podium-with-green-tropical-palm-leaves-golden-round-arch-green-wall_87521-3023.jpg".to_string() },
         // Add more products as needed
     ]);
 
     html! {
         <div class="flex flex-col items-center justify-start min-h-screen bg-gray-100">
             <header class="w-full py-4 bg-gray-800 text-white flex justify-between items-center px-8 fixed top-0 z-50">
                 <h1 class="text-2xl font-bold text-white">{"Our Shop"}</h1>
                 <div class="relative">
                     <button class="bg-blue-500 text-white py-2 px-4 rounded-full">
                         {"Cart ("} {cart.get_items().len()} {")"}
                     </button>
                     <div class="absolute right-0 mt-2 w-48 bg-white text-black rounded-lg shadow-lg">
                         <ul>
                             { for cart.get_items().iter().map(|item| html! { <li class="p-2 border-b">{item}</li> }) }
                         </ul>
                     </div>
                 </div>
             </header>
             <div class="relative w-full py-12 flex flex-col items-center mt-16 ">
                 <div class="absolute inset-0 overflow-hidden">
                     <div class="w-full h-full bg-cover bg-center" style="background-image: url('https://img.freepik.com/free-psd/black-friday-super-sale-web-banner-template_106176-1671.jpg'); filter: hue-rotate(180deg) brightness(0.7);"></div>
                     <div class="w-full h-full bg-cover bg-center" style="background-image: url('https://img.freepik.com/premium-psd/black-friday-super-sale-facebook-cover-template_106176-3157.jpg'); filter: hue-rotate(180deg) brightness(0.7);"></div>
                     <div class="w-full h-full bg-cover bg-center" style="background-image: url('https://img.freepik.com/premium-psd/black-friday-social-media-web-banner-psd-template_1013854-527.jpg'); filter: hue-rotate(180deg) brightness(0.7);"></div>
                 </div>
                 <div class="relative z-10 bg-gradient-to-r from-gray-700 to-gray-900 w-full py-12 flex flex-col items-center rounded-lg shadow-lg">
                     <h1 class="text-6xl font-extrabold mb-8 text-white drop-shadow-lg">{"Welcome to Our Shop"}</h1>
                     <p class="text-2xl mb-6 text-gray-300 drop-shadow-md">{"Discover the best products at unbeatable prices."}</p>
                     <button class="bg-white text-gray-700 hover:bg-gray-200 font-bold py-3 px-8 rounded-full shadow-lg transition duration-300 ease-in-out transform hover:scale-105">{"Shop Now"}</button>
                 </div>
             </div>
             <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 mt-10 w-full px-4 max-w-6xl">
                 { for products.iter().cloned().map(|product| html! {
                     <div class="bg-white rounded-lg shadow-md overflow-hidden transition-transform transform hover:scale-105">
                         <img src={product.image_url.clone()} alt={product.name.clone()} class="w-full h-48 object-cover" />
                         <div class="p-4">
                             <h2 class="text-xl font-semibold mb-2">{ &product.name }</h2>
                             <p class="text-gray-700 mb-3">{"Price: $"}{&product.price}</p>
                             <div class="flex justify-between items-center">
                                 <button class="bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 rounded-full flex items-center" onclick={add_to_cart.reform(move |_| product.name.clone())}>
                                     <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
                                         <path d="M16 11V9a4 4 0 10-8 0v2H5a1 1 0 000 2h1v5a2 2 0 002 2h4a2 2 0 002-2v-5h1a1 1 0 100-2h-1zm-6-2V9a2 2 0 114 0v2H10z" />
                                     </svg>
                                     {"Add to Cart"}
                                 </button>
                                 <button class="bg-green-500 hover:bg-green-700 text-white py-2 px-4 rounded-full" onclick={Callback::from({
                                     let link = link.clone();
                                     let id = product.id;
                                     move |_| link.push(&Route::ProductPage { id })
                                 })}>
                                     {"View"}
                                 </button>
                             </div>
                         </div>
                     </div>
                 }) }
             </div>
             <button class="mt-8 bg-purple-500 hover:bg-purple-700 text-white py-3 px-6 rounded-full" onclick={Callback::from(move |_| link.clone().push(&Route::ProductPage { id:products[0].id }))}>
                     {"Product View Page"}
             </button>
         </div>
     }
 }
 
 fn main() {

     yew::Renderer::<Home>::new().render();
    
    }