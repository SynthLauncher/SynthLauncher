import { LayoutThemeProps } from './layout';
import { StorePageThemeProps } from './store-page';

export interface Theme {
	storePage?: StorePageThemeProps;
	layout?: LayoutThemeProps;
}
