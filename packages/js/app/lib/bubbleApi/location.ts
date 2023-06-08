import { LatLng } from "react-native-maps";

export class MapService {
    static async get_location(group_uuid: uuid, client: uuid, before_timestamp: number, amount: number): Promise<Location[]> { return []; }
    static async get_num_location(group_uuid: uuid, client: uuid, from_timestamp: number, to_timestamp: number): Promise<number> { return 0; }
    static async send_location(group_uuid: uuid, longitude: number, latitude: number): Promise<void> {}
}

export interface IUserLocation {
    coordinate: LatLng;
    userInfo: number;
}
type uuid = string;