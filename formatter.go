package sibling

type Formatter interface {
	Format(path *Path) string
}

func NewFormatter(absoluteFlag bool) Formatter {
	if absoluteFlag {
		return new(AbsoluteFormatter)
	}
	return new(RelativeFormatter)
}

type AbsoluteFormatter struct {
}

type RelativeFormatter struct {
}

func (abs *AbsoluteFormatter) Format(path *Path) string {
	absolute, _ := path.Abs()
	return absolute.String()
}

func (abs *RelativeFormatter) Format(path *Path) string {
	return path.String()
}
