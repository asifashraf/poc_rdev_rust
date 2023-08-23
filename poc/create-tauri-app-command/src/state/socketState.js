import {
    atom,
} from 'recoil';

const socketState = atom({
    key: 'socketState', // unique ID (with respect to other atoms/selectors)
    default: false, // default value (aka initial value)
});

export default socketState;