import {
    Carousel,
    CarouselContent,
    CarouselItem,
    CarouselNext,
    CarouselPrevious,
} from "@/components/ui/carousel";
import Autoplay from "embla-carousel-autoplay";
import { Button } from "@/components/ui/button";

interface NewsItem {
    id: number | string;
    imageUrl: string;
    title: string;
    description: string;
    url: string;
}

function NewsCarousel({ autoplayDelay, items }: { autoplayDelay: number; items: NewsItem[] }) {
    return (
        <Carousel
            className="w-full"
            opts={{ loop: true, align: "center" }}
            plugins={[
                Autoplay({
                    delay: autoplayDelay,
                }),
            ]}
        >
            <CarouselContent className="h-[33vh]">
                {items.map((item) => (
                    <CarouselItem key={item.id} className="w-full h-full">
                        <div className="relative w-full h-full overflow-hidden rounded-lg shadow-md">
                            <img
                                src={item.imageUrl}
                                alt={item.title}
                                className="absolute inset-0 w-full h-full object-cover"
                            />

                            <div className="absolute bottom-0 left-0 right-0 bg-black/70 backdrop-blur-sm p-3 rounded-b-lg">
                                <h3 className="text-white font-semibold text-lg line-clamp-1">
                                    {item.title}
                                </h3>
                                {item.description && (
                                    <p className="text-white/90 text-sm mt-1 mb-2 line-clamp-2">
                                        {item.description}
                                    </p>
                                )}
                                <div>
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        className="text-xs font-medium"
                                    >
                                        Read More
                                    </Button>
                                </div>
                            </div>
                        </div>
                    </CarouselItem>
                ))}
            </CarouselContent>
            <CarouselPrevious className="left-2 z-10" />
            <CarouselNext className="right-2 z-10" />
        </Carousel>
    );
}

export { NewsCarousel };