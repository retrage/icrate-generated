//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[cfg_attr(feature = "apple", link(name = "MapKit", kind = "framework"))]
extern "C" {}
#[path = "MKAnnotation.rs"]
mod __MKAnnotation;
#[path = "MKAnnotationView.rs"]
mod __MKAnnotationView;
#[path = "MKCircle.rs"]
mod __MKCircle;
#[path = "MKCircleRenderer.rs"]
mod __MKCircleRenderer;
#[path = "MKClusterAnnotation.rs"]
mod __MKClusterAnnotation;
#[path = "MKCompassButton.rs"]
mod __MKCompassButton;
#[path = "MKDirections.rs"]
mod __MKDirections;
#[path = "MKDirectionsRequest.rs"]
mod __MKDirectionsRequest;
#[path = "MKDirectionsResponse.rs"]
mod __MKDirectionsResponse;
#[path = "MKDirectionsTypes.rs"]
mod __MKDirectionsTypes;
#[path = "MKDistanceFormatter.rs"]
mod __MKDistanceFormatter;
#[path = "MKFoundation.rs"]
mod __MKFoundation;
#[path = "MKGeoJSONSerialization.rs"]
mod __MKGeoJSONSerialization;
#[path = "MKGeodesicPolyline.rs"]
mod __MKGeodesicPolyline;
#[path = "MKGeometry.rs"]
mod __MKGeometry;
#[path = "MKGradientPolylineRenderer.rs"]
mod __MKGradientPolylineRenderer;
#[path = "MKHybridMapConfiguration.rs"]
mod __MKHybridMapConfiguration;
#[path = "MKImageryMapConfiguration.rs"]
mod __MKImageryMapConfiguration;
#[path = "MKLocalPointsOfInterestRequest.rs"]
mod __MKLocalPointsOfInterestRequest;
#[path = "MKLocalSearch.rs"]
mod __MKLocalSearch;
#[path = "MKLocalSearchCompleter.rs"]
mod __MKLocalSearchCompleter;
#[path = "MKLocalSearchRequest.rs"]
mod __MKLocalSearchRequest;
#[path = "MKLocalSearchResponse.rs"]
mod __MKLocalSearchResponse;
#[path = "MKLookAroundScene.rs"]
mod __MKLookAroundScene;
#[path = "MKLookAroundSceneRequest.rs"]
mod __MKLookAroundSceneRequest;
#[path = "MKLookAroundSnapshot.rs"]
mod __MKLookAroundSnapshot;
#[path = "MKLookAroundSnapshotOptions.rs"]
mod __MKLookAroundSnapshotOptions;
#[path = "MKLookAroundSnapshotter.rs"]
mod __MKLookAroundSnapshotter;
#[path = "MKLookAroundViewController.rs"]
mod __MKLookAroundViewController;
#[path = "MKMapCamera.rs"]
mod __MKMapCamera;
#[path = "MKMapCameraBoundary.rs"]
mod __MKMapCameraBoundary;
#[path = "MKMapCameraZoomRange.rs"]
mod __MKMapCameraZoomRange;
#[path = "MKMapConfiguration.rs"]
mod __MKMapConfiguration;
#[path = "MKMapItem.rs"]
mod __MKMapItem;
#[path = "MKMapSnapshot.rs"]
mod __MKMapSnapshot;
#[path = "MKMapSnapshotOptions.rs"]
mod __MKMapSnapshotOptions;
#[path = "MKMapSnapshotter.rs"]
mod __MKMapSnapshotter;
#[path = "MKMapView.rs"]
mod __MKMapView;
#[path = "MKMarkerAnnotationView.rs"]
mod __MKMarkerAnnotationView;
#[path = "MKMultiPoint.rs"]
mod __MKMultiPoint;
#[path = "MKMultiPolygon.rs"]
mod __MKMultiPolygon;
#[path = "MKMultiPolygonRenderer.rs"]
mod __MKMultiPolygonRenderer;
#[path = "MKMultiPolyline.rs"]
mod __MKMultiPolyline;
#[path = "MKMultiPolylineRenderer.rs"]
mod __MKMultiPolylineRenderer;
#[path = "MKOverlay.rs"]
mod __MKOverlay;
#[path = "MKOverlayPathRenderer.rs"]
mod __MKOverlayPathRenderer;
#[path = "MKOverlayRenderer.rs"]
mod __MKOverlayRenderer;
#[path = "MKPinAnnotationView.rs"]
mod __MKPinAnnotationView;
#[path = "MKPitchControl.rs"]
mod __MKPitchControl;
#[path = "MKPlacemark.rs"]
mod __MKPlacemark;
#[path = "MKPointAnnotation.rs"]
mod __MKPointAnnotation;
#[path = "MKPointOfInterestCategory.rs"]
mod __MKPointOfInterestCategory;
#[path = "MKPointOfInterestFilter.rs"]
mod __MKPointOfInterestFilter;
#[path = "MKPolygon.rs"]
mod __MKPolygon;
#[path = "MKPolygonRenderer.rs"]
mod __MKPolygonRenderer;
#[path = "MKPolyline.rs"]
mod __MKPolyline;
#[path = "MKPolylineRenderer.rs"]
mod __MKPolylineRenderer;
#[path = "MKShape.rs"]
mod __MKShape;
#[path = "MKStandardMapConfiguration.rs"]
mod __MKStandardMapConfiguration;
#[path = "MKTileOverlay.rs"]
mod __MKTileOverlay;
#[path = "MKTileOverlayRenderer.rs"]
mod __MKTileOverlayRenderer;
#[path = "MKTypes.rs"]
mod __MKTypes;
#[path = "MKUserLocation.rs"]
mod __MKUserLocation;
#[path = "MKUserLocationView.rs"]
mod __MKUserLocationView;
#[path = "MKZoomControl.rs"]
mod __MKZoomControl;
#[path = "NSUserActivity_MKMapItem.rs"]
mod __NSUserActivity_MKMapItem;

pub use self::__MKAnnotation::MKAnnotation;
pub use self::__MKAnnotationView::MKAnnotationCalloutInfoDidChangeNotification;
#[cfg(feature = "MapKit_MKAnnotationView")]
pub use self::__MKAnnotationView::MKAnnotationView;
pub use self::__MKAnnotationView::MKAnnotationViewZPriority;
pub use self::__MKAnnotationView::MKAnnotationViewZPriorityDefaultSelected;
pub use self::__MKAnnotationView::MKAnnotationViewZPriorityDefaultUnselected;
pub use self::__MKAnnotationView::MKAnnotationViewZPriorityMax;
pub use self::__MKAnnotationView::MKAnnotationViewZPriorityMin;
pub use self::__MKAnnotationView::MKFeatureDisplayPriority;
pub use self::__MKAnnotationView::MKFeatureDisplayPriorityDefaultHigh;
pub use self::__MKAnnotationView::MKFeatureDisplayPriorityDefaultLow;
pub use self::__MKAnnotationView::MKFeatureDisplayPriorityRequired;
pub use self::__MKAnnotationView::{
    MKAnnotationViewCollisionMode, MKAnnotationViewCollisionModeCircle,
    MKAnnotationViewCollisionModeNone, MKAnnotationViewCollisionModeRectangle,
};
pub use self::__MKAnnotationView::{
    MKAnnotationViewDragState, MKAnnotationViewDragStateCanceling,
    MKAnnotationViewDragStateDragging, MKAnnotationViewDragStateEnding,
    MKAnnotationViewDragStateNone, MKAnnotationViewDragStateStarting,
};
#[cfg(feature = "MapKit_MKCircle")]
pub use self::__MKCircle::MKCircle;
#[cfg(feature = "MapKit_MKCircleRenderer")]
pub use self::__MKCircleRenderer::MKCircleRenderer;
#[cfg(feature = "MapKit_MKClusterAnnotation")]
pub use self::__MKClusterAnnotation::MKClusterAnnotation;
#[cfg(feature = "MapKit_MKCompassButton")]
pub use self::__MKCompassButton::MKCompassButton;
#[cfg(feature = "MapKit_MKDirections")]
pub use self::__MKDirections::MKDirections;
pub use self::__MKDirections::MKDirectionsHandler;
pub use self::__MKDirections::MKETAHandler;
#[cfg(feature = "MapKit_MKDirectionsRequest")]
pub use self::__MKDirectionsRequest::MKDirectionsRequest;
pub use self::__MKDirectionsRequest::{
    MKDirectionsRoutePreference, MKDirectionsRoutePreferenceAny, MKDirectionsRoutePreferenceAvoid,
};
#[cfg(feature = "MapKit_MKDirectionsResponse")]
pub use self::__MKDirectionsResponse::MKDirectionsResponse;
#[cfg(feature = "MapKit_MKETAResponse")]
pub use self::__MKDirectionsResponse::MKETAResponse;
#[cfg(feature = "MapKit_MKRoute")]
pub use self::__MKDirectionsResponse::MKRoute;
#[cfg(feature = "MapKit_MKRouteStep")]
pub use self::__MKDirectionsResponse::MKRouteStep;
pub use self::__MKDirectionsTypes::{
    MKDirectionsTransportType, MKDirectionsTransportTypeAny, MKDirectionsTransportTypeAutomobile,
    MKDirectionsTransportTypeTransit, MKDirectionsTransportTypeWalking,
};
#[cfg(feature = "MapKit_MKDistanceFormatter")]
pub use self::__MKDistanceFormatter::MKDistanceFormatter;
pub use self::__MKDistanceFormatter::{
    MKDistanceFormatterUnitStyle, MKDistanceFormatterUnitStyleAbbreviated,
    MKDistanceFormatterUnitStyleDefault, MKDistanceFormatterUnitStyleFull,
};
pub use self::__MKDistanceFormatter::{
    MKDistanceFormatterUnits, MKDistanceFormatterUnitsDefault, MKDistanceFormatterUnitsImperial,
    MKDistanceFormatterUnitsImperialWithYards, MKDistanceFormatterUnitsMetric,
};
#[cfg(feature = "MapKit_MKGeoJSONDecoder")]
pub use self::__MKGeoJSONSerialization::MKGeoJSONDecoder;
#[cfg(feature = "MapKit_MKGeoJSONFeature")]
pub use self::__MKGeoJSONSerialization::MKGeoJSONFeature;
pub use self::__MKGeoJSONSerialization::MKGeoJSONObject;
#[cfg(feature = "MapKit_MKGeodesicPolyline")]
pub use self::__MKGeodesicPolyline::MKGeodesicPolyline;
pub use self::__MKGeometry::MKCoordinateForMapPoint;
pub use self::__MKGeometry::MKCoordinateRegion;
pub use self::__MKGeometry::MKCoordinateRegionForMapRect;
pub use self::__MKGeometry::MKCoordinateRegionMakeWithDistance;
pub use self::__MKGeometry::MKCoordinateSpan;
pub use self::__MKGeometry::MKMapPoint;
pub use self::__MKGeometry::MKMapPointForCoordinate;
pub use self::__MKGeometry::MKMapPointsPerMeterAtLatitude;
pub use self::__MKGeometry::MKMapRect;
pub use self::__MKGeometry::MKMapRectContainsPoint;
pub use self::__MKGeometry::MKMapRectContainsRect;
pub use self::__MKGeometry::MKMapRectInset;
pub use self::__MKGeometry::MKMapRectIntersection;
pub use self::__MKGeometry::MKMapRectIntersectsRect;
pub use self::__MKGeometry::MKMapRectNull;
pub use self::__MKGeometry::MKMapRectOffset;
pub use self::__MKGeometry::MKMapRectRemainder;
pub use self::__MKGeometry::MKMapRectSpans180thMeridian;
pub use self::__MKGeometry::MKMapRectUnion;
pub use self::__MKGeometry::MKMapRectWorld;
pub use self::__MKGeometry::MKMapSize;
pub use self::__MKGeometry::MKMapSizeWorld;
pub use self::__MKGeometry::MKMetersBetweenMapPoints;
pub use self::__MKGeometry::MKMetersPerMapPointAtLatitude;
pub use self::__MKGeometry::MKZoomScale;
#[cfg(feature = "MapKit_MKGradientPolylineRenderer")]
pub use self::__MKGradientPolylineRenderer::MKGradientPolylineRenderer;
#[cfg(feature = "MapKit_MKHybridMapConfiguration")]
pub use self::__MKHybridMapConfiguration::MKHybridMapConfiguration;
#[cfg(feature = "MapKit_MKImageryMapConfiguration")]
pub use self::__MKImageryMapConfiguration::MKImageryMapConfiguration;
#[cfg(feature = "MapKit_MKLocalPointsOfInterestRequest")]
pub use self::__MKLocalPointsOfInterestRequest::MKLocalPointsOfInterestRequest;
pub use self::__MKLocalPointsOfInterestRequest::MKPointsOfInterestRequestMaxRadius;
#[cfg(feature = "MapKit_MKLocalSearch")]
pub use self::__MKLocalSearch::MKLocalSearch;
pub use self::__MKLocalSearch::MKLocalSearchCompletionHandler;
#[cfg(feature = "MapKit_MKLocalSearchCompleter")]
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompleter;
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompleterDelegate;
#[cfg(feature = "MapKit_MKLocalSearchCompletion")]
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompletion;
pub use self::__MKLocalSearchCompleter::{
    MKLocalSearchCompleterResultType, MKLocalSearchCompleterResultTypeAddress,
    MKLocalSearchCompleterResultTypePointOfInterest, MKLocalSearchCompleterResultTypeQuery,
};
pub use self::__MKLocalSearchCompleter::{
    MKSearchCompletionFilterType, MKSearchCompletionFilterTypeLocationsAndQueries,
    MKSearchCompletionFilterTypeLocationsOnly,
};
#[cfg(feature = "MapKit_MKLocalSearchRequest")]
pub use self::__MKLocalSearchRequest::MKLocalSearchRequest;
pub use self::__MKLocalSearchRequest::{
    MKLocalSearchResultType, MKLocalSearchResultTypeAddress, MKLocalSearchResultTypePointOfInterest,
};
#[cfg(feature = "MapKit_MKLocalSearchResponse")]
pub use self::__MKLocalSearchResponse::MKLocalSearchResponse;
#[cfg(feature = "MapKit_MKLookAroundScene")]
pub use self::__MKLookAroundScene::MKLookAroundScene;
#[cfg(feature = "MapKit_MKLookAroundSceneRequest")]
pub use self::__MKLookAroundSceneRequest::MKLookAroundSceneRequest;
#[cfg(feature = "MapKit_MKLookAroundSnapshot")]
pub use self::__MKLookAroundSnapshot::MKLookAroundSnapshot;
#[cfg(feature = "MapKit_MKLookAroundSnapshotOptions")]
pub use self::__MKLookAroundSnapshotOptions::MKLookAroundSnapshotOptions;
#[cfg(feature = "MapKit_MKLookAroundSnapshotter")]
pub use self::__MKLookAroundSnapshotter::MKLookAroundSnapshotter;
#[cfg(feature = "MapKit_MKLookAroundViewController")]
pub use self::__MKLookAroundViewController::MKLookAroundViewController;
pub use self::__MKLookAroundViewController::MKLookAroundViewControllerDelegate;
pub use self::__MKLookAroundViewController::{
    MKLookAroundBadgePosition, MKLookAroundBadgePositionBottomTrailing,
    MKLookAroundBadgePositionTopLeading, MKLookAroundBadgePositionTopTrailing,
};
#[cfg(feature = "MapKit_MKMapCamera")]
pub use self::__MKMapCamera::MKMapCamera;
#[cfg(feature = "MapKit_MKMapCameraBoundary")]
pub use self::__MKMapCameraBoundary::MKMapCameraBoundary;
pub use self::__MKMapCameraZoomRange::MKMapCameraZoomDefault;
#[cfg(feature = "MapKit_MKMapCameraZoomRange")]
pub use self::__MKMapCameraZoomRange::MKMapCameraZoomRange;
#[cfg(feature = "MapKit_MKMapConfiguration")]
pub use self::__MKMapConfiguration::MKMapConfiguration;
pub use self::__MKMapConfiguration::{
    MKMapElevationStyle, MKMapElevationStyleFlat, MKMapElevationStyleRealistic,
};
pub use self::__MKMapItem::MKLaunchOptionsCameraKey;
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeDefault;
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeDriving;
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeKey;
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeTransit;
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeWalking;
pub use self::__MKMapItem::MKLaunchOptionsMapCenterKey;
pub use self::__MKMapItem::MKLaunchOptionsMapSpanKey;
pub use self::__MKMapItem::MKLaunchOptionsMapTypeKey;
pub use self::__MKMapItem::MKLaunchOptionsShowsTrafficKey;
#[cfg(feature = "MapKit_MKMapItem")]
pub use self::__MKMapItem::MKMapItem;
pub use self::__MKMapItem::MKMapItemTypeIdentifier;
#[cfg(feature = "MapKit_MKMapSnapshot")]
pub use self::__MKMapSnapshot::MKMapSnapshot;
#[cfg(feature = "MapKit_MKMapSnapshotOptions")]
pub use self::__MKMapSnapshotOptions::MKMapSnapshotOptions;
pub use self::__MKMapSnapshotter::MKMapSnapshotCompletionHandler;
#[cfg(feature = "MapKit_MKMapSnapshotter")]
pub use self::__MKMapSnapshotter::MKMapSnapshotter;
#[cfg(feature = "MapKit_MKMapView")]
pub use self::__MKMapView::MKMapView;
pub use self::__MKMapView::MKMapViewDefaultAnnotationViewReuseIdentifier;
pub use self::__MKMapView::MKMapViewDefaultClusterAnnotationViewReuseIdentifier;
pub use self::__MKMapView::MKMapViewDelegate;
pub use self::__MKMapView::{MKOverlayLevel, MKOverlayLevelAboveLabels, MKOverlayLevelAboveRoads};
pub use self::__MKMapView::{
    MKUserTrackingMode, MKUserTrackingModeFollow, MKUserTrackingModeFollowWithHeading,
    MKUserTrackingModeNone,
};
#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
pub use self::__MKMarkerAnnotationView::MKMarkerAnnotationView;
#[cfg(feature = "MapKit_MKMultiPoint")]
pub use self::__MKMultiPoint::MKMultiPoint;
#[cfg(feature = "MapKit_MKMultiPolygon")]
pub use self::__MKMultiPolygon::MKMultiPolygon;
#[cfg(feature = "MapKit_MKMultiPolygonRenderer")]
pub use self::__MKMultiPolygonRenderer::MKMultiPolygonRenderer;
#[cfg(feature = "MapKit_MKMultiPolyline")]
pub use self::__MKMultiPolyline::MKMultiPolyline;
#[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
pub use self::__MKMultiPolylineRenderer::MKMultiPolylineRenderer;
pub use self::__MKOverlay::MKOverlay;
#[cfg(feature = "MapKit_MKOverlayPathRenderer")]
pub use self::__MKOverlayPathRenderer::MKOverlayPathRenderer;
#[cfg(feature = "MapKit_MKOverlayRenderer")]
pub use self::__MKOverlayRenderer::MKOverlayRenderer;
pub use self::__MKOverlayRenderer::MKRoadWidthAtZoomScale;
#[cfg(feature = "MapKit_MKPinAnnotationView")]
pub use self::__MKPinAnnotationView::MKPinAnnotationView;
pub use self::__MKPinAnnotationView::{
    MKPinAnnotationColor, MKPinAnnotationColorGreen, MKPinAnnotationColorPurple,
    MKPinAnnotationColorRed,
};
#[cfg(feature = "MapKit_MKPitchControl")]
pub use self::__MKPitchControl::MKPitchControl;
#[cfg(feature = "MapKit_MKPlacemark")]
pub use self::__MKPlacemark::MKPlacemark;
#[cfg(feature = "MapKit_MKPointAnnotation")]
pub use self::__MKPointAnnotation::MKPointAnnotation;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategory;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryATM;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryAirport;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryAmusementPark;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryAquarium;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryBakery;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryBank;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryBeach;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryBrewery;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryCafe;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryCampground;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryCarRental;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryEVCharger;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryFireStation;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryFitnessCenter;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryFoodMarket;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryGasStation;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryHospital;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryHotel;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryLaundry;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryLibrary;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryMarina;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryMovieTheater;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryMuseum;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryNationalPark;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryNightlife;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPark;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryParking;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPharmacy;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPolice;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPostOffice;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPublicTransport;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryRestaurant;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryRestroom;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategorySchool;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryStadium;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryStore;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryTheater;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryUniversity;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryWinery;
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryZoo;
#[cfg(feature = "MapKit_MKPointOfInterestFilter")]
pub use self::__MKPointOfInterestFilter::MKPointOfInterestFilter;
#[cfg(feature = "MapKit_MKPolygon")]
pub use self::__MKPolygon::MKPolygon;
#[cfg(feature = "MapKit_MKPolygonRenderer")]
pub use self::__MKPolygonRenderer::MKPolygonRenderer;
#[cfg(feature = "MapKit_MKPolyline")]
pub use self::__MKPolyline::MKPolyline;
#[cfg(feature = "MapKit_MKPolylineRenderer")]
pub use self::__MKPolylineRenderer::MKPolylineRenderer;
#[cfg(feature = "MapKit_MKShape")]
pub use self::__MKShape::MKShape;
#[cfg(feature = "MapKit_MKStandardMapConfiguration")]
pub use self::__MKStandardMapConfiguration::MKStandardMapConfiguration;
pub use self::__MKStandardMapConfiguration::{
    MKStandardMapEmphasisStyle, MKStandardMapEmphasisStyleDefault, MKStandardMapEmphasisStyleMuted,
};
#[cfg(feature = "MapKit_MKTileOverlay")]
pub use self::__MKTileOverlay::MKTileOverlay;
pub use self::__MKTileOverlay::MKTileOverlayPath;
#[cfg(feature = "MapKit_MKTileOverlayRenderer")]
pub use self::__MKTileOverlayRenderer::MKTileOverlayRenderer;
pub use self::__MKTypes::MKErrorDomain;
pub use self::__MKTypes::{
    MKErrorCode, MKErrorDecodingFailed, MKErrorDirectionsNotFound, MKErrorLoadingThrottled,
    MKErrorPlacemarkNotFound, MKErrorServerFailure, MKErrorUnknown,
};
pub use self::__MKTypes::{
    MKFeatureVisibility, MKFeatureVisibilityAdaptive, MKFeatureVisibilityHidden,
    MKFeatureVisibilityVisible,
};
pub use self::__MKTypes::{
    MKMapType, MKMapTypeHybrid, MKMapTypeHybridFlyover, MKMapTypeMutedStandard, MKMapTypeSatellite,
    MKMapTypeSatelliteFlyover, MKMapTypeStandard,
};
#[cfg(feature = "MapKit_MKUserLocation")]
pub use self::__MKUserLocation::MKUserLocation;
#[cfg(feature = "MapKit_MKUserLocationView")]
pub use self::__MKUserLocationView::MKUserLocationView;
#[cfg(feature = "MapKit_MKZoomControl")]
pub use self::__MKZoomControl::MKZoomControl;
