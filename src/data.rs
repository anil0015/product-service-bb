use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Apple MacBook Pro 14\"".to_string(),
            price: 2499.99,
            description: "Supercharged by M3 Pro. The MacBook Pro takes power and efficiency further than ever. It delivers exceptional performance whether plugged in or not.".to_string(),
            image: "/macbook.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Sony WH-1000XM5 Headphones".to_string(),
            price: 498.00,
            description: "The best noise cancelling headphones on the market. Features industry-leading noise cancellation, exceptional sound quality, and crystal-clear hands-free calling.".to_string(),
            image: "/headphones.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Samsung Galaxy S24 Ultra".to_string(),
            price: 1799.99,
            description: "Unleash new ways to create, connect, and more with the Galaxy S24 Ultra. The new era of mobile AI is here.".to_string(),
            image: "/samsung.jpg".to_string()
        },
        Product {
            id: 4,
            name: "LG OLED C3 Series TV".to_string(),
            price: 2199.99,
            description: "The LG OLED evo C-Series is powered by the a9 AI Processor Gen6, made exclusively for LG OLED, for beautiful picture and performance.".to_string(),
            image: "/tv.jpg".to_string()
        },
        Product {
            id: 5,
            name: "PlayStation 5 Console".to_string(),
            price: 649.99,
            description: "Experience lightning-fast loading with an ultra-high-speed SSD, deeper immersion with haptic feedback, adaptive triggers, and 3D Audio.".to_string(),
            image: "/ps5.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Nintendo Switch OLED".to_string(),
            price: 449.99,
            description: "Feast your eyes on vivid colors and crisp contrast when you play on-the-go. See the difference the vibrant screen makes, whether you're racing at top speed or battling enemies.".to_string(),
            image: "/switch.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Apple iPad Air".to_string(),
            price: 799.00,
            description: "iPad Air. With an immersive 10.9-inch Liquid Retina display. The breakthrough M1 chip delivers faster performance, making iPad Air a creative and mobile gaming powerhouse.".to_string(),
            image: "/ipad.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Dell XPS 13 Laptop".to_string(),
            price: 1399.99,
            description: "The XPS 13 is thin and light, with a long battery life, making it the perfect companion for people on the go.".to_string(),
            image: "/dell.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Logitech MX Master 3S".to_string(),
            price: 129.99,
            description: "Meet MX Master 3S – an iconic mouse remastered. Feel every moment of your workflow with even more precision, tactility, and performance.".to_string(),
            image: "/mouse.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Rocketfish™ 4K HDMI Cable".to_string(),
            price: 29.99,
            description: "Connect your devices with this high-speed HDMI cable. It supports 4K Ultra HD video and HDR for a stunning visual experience.".to_string(),
            image: "/hdmi.jpg".to_string()
        }
    ]
}