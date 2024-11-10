import { writable } from 'svelte/store';

// Stores the list of selected breeds
export const selectedBreeds = writable<string[][]>([]);
// Stores the content of the lightbox popup
export const popupImage = writable<{src: string, alt: string} | null>(null);
