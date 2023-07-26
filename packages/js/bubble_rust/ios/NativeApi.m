#import "NativeApi.h"
#import <CoreLocation/CoreLocation.h>
#import "../rust.h"

@interface LocationManager () <CLLocationManagerDelegate>

@property(strong, nonatomic) CLLocationManager *locationManager;

@end

@implementation LocationManager

- (instancetype)init {
    if (self = [super init]) {
        self.locationManager = [[CLLocationManager alloc] init];
        NSLog(@"LocationManager lastLocation: %@", [self.locationManager location]);
        self.locationManager.delegate = self;

    }
    return self;
}


- (void)requestAlwaysAuthorization {
    [self.locationManager requestAlwaysAuthorization];
}

- (void)subscribe {
    NSLog(@"LocationManager subscribe");
    [self.locationManager startMonitoringSignificantLocationChanges];
}

- (void)unsubscribe {
    [self.locationManager stopMonitoringSignificantLocationChanges];
}

- (void)locationManagerDidChangeAuthorization:(CLLocationManager *)manager {
    NSLog(@"LocationManager didChangeAuthorizationStatus: %d", manager.authorizationStatus);
}

- (void)locationManager:(CLLocationManager *)manager didUpdateLocations:(NSArray*)locations {
    NSMutableArray *updates = [NSMutableArray array];
    for (CLLocation *location in locations) {
        NSMutableDictionary *update = [NSMutableDictionary dictionary];
        update[@"longitude"] = @(location.coordinate.longitude);
        update[@"latitude"] = @(location.coordinate.latitude);
        update[@"timestamp"] = @(location.timestamp.timeIntervalSince1970);
        if (location.altitude) {
            update[@"altitude"] = @(location.altitude);
        }
        if (location.floor) {
            update[@"floor"] = @(location.floor.level);
        }
        if (location.course) {
            update[@"course"] = @(location.course);
        }
        if (location.horizontalAccuracy) {
            update[@"horizontal_accuracy"] = @(location.horizontalAccuracy);
        }
        if (location.verticalAccuracy) {
            update[@"vertical_accuracy"] = @(location.verticalAccuracy);
        }
        if (location.courseAccuracy) {
            update[@"course_accuracy"] = @(location.courseAccuracy);
        }
        if (location.speed) {
            update[@"speed"] = @(location.speed);
        }
        if (location.speedAccuracy) {
            update[@"speed_accuracy"] = @(location.speedAccuracy);
        }
        [updates addObject:update];
    }
    
    NSArray *paths = NSSearchPathForDirectoriesInDomains(NSLibraryDirectory, NSUserDomainMask, YES);
    NSString *dir = [paths firstObject];

    NSMutableDictionary *updates_dict = [NSMutableDictionary dictionary];
    updates_dict[@"data_directory"] = dir;
    updates_dict[@"updates"] = updates;

    NSData *data = [NSJSONSerialization dataWithJSONObject:updates_dict options:0 error:nil];
    NSString *json = [[NSString alloc] initWithData:data encoding:NSUTF8StringEncoding];

    background_location_update(json.UTF8String);
}

- (void)locationManager:(CLLocationManager *)manager didFailWithError:(NSError *)error {
    NSLog(@"LocationManager didFailWithError: %@", error);
}


const void *create_location_manager() {
    // this took fucking forever to figure out
    // i don't really understand objective-c async model to explain this fully but for some reason if the object isn't allocated on the main thread
    // the delegate methods never get called
    // so we do this bullshit
    __block LocationManager *locationManager;
    dispatch_sync(dispatch_get_main_queue(), ^{
        locationManager = [[LocationManager alloc] init];
    });
    NSLog(@"create_location_manager: %p", locationManager);
    return CFBridgingRetain(locationManager);
}

void request_location_permissions(const void *location_manager) {
    LocationManager *locationManager = (__bridge LocationManager *) location_manager;
    [locationManager requestAlwaysAuthorization];
}

bool has_location_permissions() {
    CLAuthorizationStatus status = [CLLocationManager authorizationStatus];
    NSLog(@"has_location_permissions: %d", status);
    return status == kCLAuthorizationStatusAuthorizedAlways || status == kCLAuthorizationStatusAuthorizedWhenInUse;
}

void subscribe_to_location_updates(const void *location_manager) {
    LocationManager *locationManager = (__bridge LocationManager *) location_manager;
    [locationManager subscribe];
}

void unsubscribe_from_location_updates(const void *location_manager) {
    LocationManager *locationManager = (__bridge LocationManager *) location_manager;
    [locationManager unsubscribe];
}

void destroy_location_manager(void *location_manager) {
    CFBridgingRelease(location_manager);
}

@end


