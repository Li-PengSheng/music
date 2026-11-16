import { mount } from 'svelte';
import '../../src/app.css';
import PrimitivesFixture from './PrimitivesFixture.svelte';
mount(PrimitivesFixture, { target: document.getElementById('app')! });
