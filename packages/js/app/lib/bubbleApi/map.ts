import { LatLng } from 'react-native-maps';

export class MapService {
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    static async getAllBubbles() {}

    // eslint-disable-next-line @typescript-eslint/no-empty-function
    static async getBubbleLocations(bubbleId: number) {}
}

export interface IBasicUserInfo {
    id: number;
    name: string;
}

export interface IBubbleInfo {
    id: number;
    name: string;
    users: IBasicUserInfo[];
}

export interface IUserLocation {
    coordinate: LatLng;
    userInfo: IBasicUserInfo;
}
