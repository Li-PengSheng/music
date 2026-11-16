import { mount } from 'svelte';
import '../../src/app.css';
import SelectFixture from './SelectFixture.svelte';

mount(SelectFixture, { target: document.getElementById('app')! });
