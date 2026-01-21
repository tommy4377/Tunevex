import { writable } from 'svelte/store';
import type { TweakCategory } from './types';

export const activeCategory = writable<TweakCategory>('Home');
export const selectedTweaks = writable<Set<string>>(new Set());
