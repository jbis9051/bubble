import { LatLng } from 'react-native-maps';

export class MapService {
    static async getAllBubbles() {}

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
