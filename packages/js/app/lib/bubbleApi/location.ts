import { LatLng } from 'react-native-maps';

function generateSampleLocations(numLocations: number): LocationWithTime[] {
    const locations: LocationWithTime[] = [];

    const US_BOUNDARIES = {
        latitude: { min: 24.396308, max: 49.384358 },
        longitude: { min: -125.0, max: -66.93457 },
    };

    for (let i = 0; i < numLocations; i++) {
        const latitude =
            Math.random() *
                (US_BOUNDARIES.latitude.max - US_BOUNDARIES.latitude.min) +
            US_BOUNDARIES.latitude.min;
        const longitude =
            Math.random() *
                (US_BOUNDARIES.longitude.max - US_BOUNDARIES.longitude.min) +
            US_BOUNDARIES.longitude.min;

        const timestamp = Date.now() - Math.floor(Math.random() * 86400000);

        const location: LocationWithTime = {
            coordinate: { latitude, longitude },
            timestamp,
        };

        locations.push(location);
    }

    return locations;
}

export class MapService {
    static async get_location(
        group_uuid: uuid,
        client: uuid,
        before_timestamp: number,
        amount: number
    ): Promise<LocationWithTime[]> {
        return generateSampleLocations(10);
    }

    static async get_num_location(
        group_uuid: uuid,
        client: uuid,
        from_timestamp: number,
        to_timestamp: number
    ): Promise<number> {
        return 0;
    }

    static async send_location(
        group_uuid: uuid,
        longitude: number,
        latitude: number
    ): Promise<void> {}
}

export interface LocationWithTime {
    coordinate: LatLng;
    timestamp: number;
}

export interface IUserLocation {
    coordinate: LatLng;
    userInfo: uuid;
}
type uuid = string;
